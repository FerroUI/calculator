//! Code-behind adapters for the retained XAML trees; calculations stay in C++.
pub(super) use crate::{
    date_calculator,
    engine::{Command, Engine, Mode, Result, Snapshot},
};
pub(super) use chrono::Datelike;
pub(super) use ferroui::base::ObjectType;
pub(super) use ferroui::controls::primitives::UniformGrid;
pub(super) use ferroui::{
    base::{
        input::{
            InputElement, Key, KeyModifiers, TextInputEventArgs, platform::ClipboardExtensions,
        },
        styling::ThemeVariant,
        utilities::DateTime,
    },
    controls::{
        Border, CalendarDatePicker, ComboBox, ContentControl, Grid, NavigationView,
        NavigationViewItem, NavigationViewItemHeader, StackPanel, TextBox, UserControl,
    },
    markup_xaml_loader::FerroRuntimeXamlLoader,
    prelude::*,
};
pub(super) use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Calculator(Mode),
    Converter(i32),
    Date,
    Graphing,
    Currency,
    Settings,
}

pub struct MainPage {
    pub root: Ref<Grid>,
    pub engine: RefCell<Engine>,
    pub page: Cell<Page>,
    pub content: RefCell<Option<Ref<UserControl>>>,
    pub operators: RefCell<Option<Ref<UserControl>>>,
    pub(super) number_pad: RefCell<Option<Ref<UserControl>>>,
    pub(super) angles: RefCell<Option<Ref<UserControl>>>,
    pub(super) memory_tab: Cell<bool>,
    pub(super) radix: Cell<u32>,
    pub(super) word: Cell<usize>,
    pub(super) angle: Cell<usize>,
    pub(super) updating: Cell<bool>,
    pub(super) bits: RefCell<Vec<Ref<Button>>>,
    navigation_items: RefCell<Vec<(Page, Ref<NavigationViewItem>)>>,
}

pub(super) fn load<T: ObjectType>(xaml: &str) -> Ref<T> {
    FerroRuntimeXamlLoader::parse_as(xaml, None).expect("ported XAML")
}
pub(super) fn label(text: &str, size: f64) -> Ref<TextBlock> {
    let t = TextBlock::new();
    t.set_text(Some(text));
    t.set_font_size(size);
    t.set_text_wrapping(ferroui::base::media::TextWrapping::Wrap);
    t
}
pub(super) fn button(text: &str) -> Ref<Button> {
    let b = Button::new();
    b.classes().add("calc");
    b.classes().add("caption");
    b.set_content(Some(Rc::new(text.to_owned())));
    b
}

#[derive(serde::Deserialize)]
pub(super) struct KeypadBinding {
    pub(super) name: String,
    pub(super) command: String,
    #[serde(default)]
    pub(super) shortcuts: HashMap<String, String>,
}
pub(super) fn bindings() -> &'static HashMap<String, Vec<KeypadBinding>> {
    static BINDINGS: std::sync::OnceLock<HashMap<String, Vec<KeypadBinding>>> =
        std::sync::OnceLock::new();
    BINDINGS.get_or_init(|| {
        serde_json::from_str(include_str!("commands.json")).expect("command metadata")
    })
}

impl MainPage {
    pub fn new() -> Result<Rc<Self>> {
        let this = Rc::new(Self {
            root: load(include_str!("MainPage.xaml")),
            engine: RefCell::new(Engine::new()?),
            page: Cell::new(Page::Calculator(Mode::Standard)),
            content: RefCell::new(None),
            operators: RefCell::new(None),
            number_pad: RefCell::new(None),
            angles: RefCell::new(None),
            memory_tab: Cell::new(false),
            radix: Cell::new(10),
            word: Cell::new(0),
            angle: Cell::new(0),
            updating: Cell::new(false),
            bits: RefCell::new(Vec::new()),
            navigation_items: RefCell::new(Vec::new()),
        });
        this.bind_shell("ThemeButton", |_| Self::toggle_theme());
        this.bind_shell("HistoryButton", |this| {
            let pane = this.root.get_control::<Border>("HistoryPane");
            pane.set_is_visible(!pane.is_visible());
            this.refresh();
        });
        this.bind_shell("HistoryTab", |this| {
            this.memory_tab.set(false);
            this.refresh();
        });
        this.bind_shell("MemoryTab", |this| {
            this.memory_tab.set(true);
            this.refresh();
        });
        this.bind_shell("ClearHistoryMemory", |this| {
            this.apply(|e| {
                if this.memory_tab.get() {
                    e.clear_memory()
                } else {
                    e.clear_history()
                }
            })
        });
        let nav = this.root.get_control::<NavigationView>("NavView");
        // CalculatorIcons includes the Settings glyph; use it off Windows too.
        let settings_icon: Ref<ferroui::controls::FontIcon> = load(
            r#"<FontIcon xmlns="https://github.com/ferroui" FontFamily="fonts:Calculator#Calculator Fluent Icons" Glyph="&#xE713;"/>"#,
        );
        nav.settings_item().set_icon(Some(settings_icon.upcast()));
        let header = NavigationViewItemHeader::new();
        header.set_content(Some(Rc::new("Calculator".to_owned())));
        nav.menu_items().add(Some(Control::boxed(header)));
        for (title, page) in [
            ("Standard", Page::Calculator(Mode::Standard)),
            ("Scientific", Page::Calculator(Mode::Scientific)),
            ("Graphing", Page::Graphing),
            ("Programmer", Page::Calculator(Mode::Programmer)),
            ("Date calculation", Page::Date),
        ] {
            this.add_navigation(&nav, title, page);
        }
        let header = NavigationViewItemHeader::new();
        header.set_content(Some(Rc::new("Converter".to_owned())));
        nav.menu_items().add(Some(Control::boxed(header)));
        this.add_navigation(&nav, "Currency", Page::Currency);
        let categories = this.engine.borrow().snapshot()?.converter.categories;
        for category in categories.into_iter().filter(|c| c.id != 0) {
            this.add_navigation(&nav, &category.name, Page::Converter(category.id));
        }
        let weak = Rc::downgrade(&this);
        nav.selection_changed(move |args| {
            if let Some(this) = weak.upgrade() {
                let page = if args.is_settings_selected() {
                    Some(Page::Settings)
                } else {
                    args.selected_item_container()
                        .and_then(|item| item.tag())
                        .and_then(|tag| tag.downcast_ref::<Page>().copied())
                };
                if let Some(page) = page
                    && this.page.get() != page
                {
                    this.navigate(page);
                }
            }
        });
        let weak_nav = nav.downgrade();
        nav.item_invoked(move |_| {
            if let Some(nav) = weak_nav.upgrade() {
                nav.set_is_pane_open(false);
            }
        });
        this.navigate(Page::Calculator(Mode::Standard));
        Ok(this)
    }
    fn add_navigation(&self, nav: &NavigationView, title: &str, page: Page) {
        let item = NavigationViewItem::new();
        item.set_content(Some(Rc::new(title.to_owned())));
        item.set_tag(Some(Rc::new(page)));
        let glyphs: HashMap<String, String> = serde_json::from_str(include_str!("navigation.json"))
            .expect("original navigation glyphs");
        let source_title = if page == Page::Date { "Date" } else { title };
        if let Some(glyph) = glyphs
            .iter()
            .find_map(|(name, glyph)| name.eq_ignore_ascii_case(source_title).then_some(glyph))
        {
            let icon = ferroui::controls::FontIcon::new();
            icon.set_font_family(ferroui::base::media::FontFamily::from(
                "fonts:Calculator#Calculator Fluent Icons",
            ));
            icon.set_glyph(glyph.clone());
            item.set_icon(Some(icon.upcast()));
        }
        nav.menu_items().add(Some(Control::boxed(&item)));
        self.navigation_items.borrow_mut().push((page, item));
    }
    pub fn toggle_theme() {
        if let Some(app) = Application::current() {
            let dark = app.actual_theme_variant() == Some(ThemeVariant::dark());
            app.set_requested_theme_variant(Some(if dark {
                ThemeVariant::light()
            } else {
                ThemeVariant::dark()
            }));
        }
    }
    pub(super) fn bind_shell(self: &Rc<Self>, name: &str, f: impl Fn(&Rc<Self>) + 'static) {
        self.bind(&self.root.clone().upcast(), name, f);
    }
    pub(super) fn bind(
        self: &Rc<Self>,
        root: &Ref<Control>,
        name: &str,
        f: impl Fn(&Rc<Self>) + 'static,
    ) {
        if let Some(b) = root.find_control::<Button>(name) {
            let weak = Rc::downgrade(self);
            b.click(move |_, _| {
                if let Some(this) = weak.upgrade() {
                    f(&this);
                }
            });
        }
    }
    pub fn apply(self: &Rc<Self>, f: impl FnOnce(&mut Engine) -> Result<()>) {
        let result = f(&mut self.engine.borrow_mut());
        match result {
            Ok(()) => self.refresh(),
            Err(e) => self.show_error(&e.to_string()),
        }
    }
    pub fn command(self: &Rc<Self>, command: Command) {
        self.apply(|e| e.send(command));
    }
    pub(super) fn show_error(&self, message: &str) {
        eprintln!("FerroCalc: {message}");
        if !matches!(self.page.get(), Page::Calculator(_)) {
            return;
        }
        if let Some(page) = self.content.borrow().as_ref()
            && let Some(t) = page.find_control::<TextBlock>("ExpressionDisplay")
        {
            t.set_text(Some(message));
        }
    }
    pub fn navigate(self: &Rc<Self>, page: Page) {
        let navigation = self.root.get_control::<NavigationView>("NavView");
        navigation.set_is_pane_open(false);
        self.operators.borrow_mut().take();
        self.angles.borrow_mut().take();
        self.number_pad.borrow_mut().take();
        self.bits.borrow_mut().clear();
        self.page.set(page);
        let selected = if page == Page::Settings {
            Some(navigation.settings_item())
        } else {
            self.navigation_items
                .borrow()
                .iter()
                .find(|(candidate, _)| *candidate == page)
                .map(|(_, item)| item.clone())
        };
        navigation.set_selected_item(selected.map(Control::boxed));
        let (title, view) = match page {
            Page::Calculator(mode) => {
                if let Err(error) = self.engine.borrow_mut().set_mode(mode) {
                    self.show_error(&error.to_string());
                    return;
                }
                self.radix.set(10);
                self.word.set(0);
                self.angle.set(0);
                (mode.title().to_owned(), self.calculator(mode))
            }
            Page::Converter(id) => {
                if let Err(error) = self.engine.borrow_mut().set_category(id) {
                    self.show_error(&error.to_string());
                    return;
                }
                let state = self.engine.borrow().snapshot().expect("native snapshot");
                let title = state
                    .converter
                    .categories
                    .iter()
                    .find(|c| c.id == id)
                    .map(|c| c.name.clone())
                    .unwrap_or_default();
                (title, self.converter())
            }
            Page::Date => ("Date calculation".into(), self.date()),
            Page::Graphing => (
                "Graphing".into(),
                Self::message(
                    "Graphing is unavailable",
                    "Microsoft's open-source Calculator ships a mock graphing engine. The original GraphingCalculator XAML and graph interfaces are preserved in upstream/calculator. Connecting a real graph engine and adapting the graph UI remain open port work.",
                ),
            ),
            Page::Currency => {
                if let Err(error) = self.engine.borrow_mut().set_category(0) {
                    self.show_error(&error.to_string());
                    return;
                }
                let view = self.converter();
                view.get_control::<TextBlock>("CurrencyNotice")
                    .set_is_visible(true);
                ("Currency (mock rates)".into(), view)
            }
            Page::Settings => {
                let view = Self::message(
                    "Calculator appearance",
                    "Use the appearance button to switch between the light and dark Calculator resources. FerroUI's WinUI theme supplies the controls.",
                );
                let panel = view
                    .content()
                    .unwrap()
                    .downcast_ref::<Ref<Control>>()
                    .and_then(|c| c.cast::<StackPanel>());
                if let Some(panel) = panel {
                    let b = button("Toggle light / dark");
                    b.click(|_, _| Self::toggle_theme());
                    panel.children().add(b);
                }
                ("Settings".into(), view)
            }
        };
        self.root
            .get_control::<TextBlock>("Header")
            .set_text(Some(&title));
        self.root
            .get_control::<ContentControl>("ContentFrame")
            .set_content(Some(Control::boxed(view.clone())));
        *self.content.borrow_mut() = Some(view);
        let calculator = matches!(page, Page::Calculator(_));
        self.root
            .get_control::<Button>("HistoryButton")
            .set_is_visible(calculator);
        if !calculator {
            self.root
                .get_control::<Border>("HistoryPane")
                .set_is_visible(false);
        }
        self.refresh();
    }
    pub(super) fn message(title: &str, text: &str) -> Ref<UserControl> {
        let view = UserControl::new();
        let panel = StackPanel::new();
        panel.set_margin(Thickness::uniform(20.0));
        panel.set_spacing(16.0);
        panel.children().add(label(title, 24.0));
        panel.children().add(label(text, 16.0));
        view.set_content(Some(Control::boxed(panel)));
        view
    }
    pub fn refresh(self: &Rc<Self>) {
        if self.updating.replace(true) {
            return;
        }
        self.refresh_inner();
        self.updating.set(false);
    }
    pub(super) fn refresh_inner(self: &Rc<Self>) {
        let snapshot = match self.engine.borrow().snapshot() {
            Ok(s) => s,
            Err(e) => {
                self.show_error(&e.to_string());
                return;
            }
        };
        let Some(view) = self.content.borrow().clone() else {
            return;
        };
        match self.page.get() {
            Page::Calculator(mode) => {
                view.get_control::<TextBlock>("CalculatorResults")
                    .set_text(Some(&snapshot.display));
                view.get_control::<TextBlock>("ExpressionDisplay")
                    .set_text(Some(&snapshot.expression));
                for name in ["MemoryClear", "MemoryRecall"] {
                    view.get_control::<Button>(name)
                        .set_is_enabled(!snapshot.memory.is_empty() && !snapshot.error);
                }
                for name in ["MemoryAdd", "MemorySubtract", "MemoryStore"] {
                    view.get_control::<Button>(name)
                        .set_is_enabled(!snapshot.error);
                }
                for root in [
                    self.operators.borrow().clone(),
                    self.number_pad.borrow().clone(),
                ]
                .into_iter()
                .flatten()
                {
                    self.update_keypad(&root, &snapshot, mode);
                }
                if let Some(angle) = self.angles.borrow().as_ref() {
                    angle
                        .get_control::<Button>("DegreeButton")
                        .set_content(Some(Rc::new(
                            ["DEG", "RAD", "GRAD"][self.angle.get()].to_owned(),
                        )));
                }
                if mode == Mode::Programmer {
                    for (i, name) in ["HexButton", "DecimalButton", "OctButton", "BinaryButton"]
                        .iter()
                        .enumerate()
                    {
                        let button = view.get_control::<ferroui::controls::RadioButton>(name);
                        button.set_content(Some(Rc::new(snapshot.radices[i].clone())));
                        button.set_is_checked(Some(self.radix.get() == [16, 10, 8, 2][i]));
                    }
                    view.get_control::<Button>("WordSizeButton")
                        .set_content(Some(Rc::new(
                            ["QWORD", "DWORD", "WORD", "BYTE"][self.word.get()].to_owned(),
                        )));
                    let binary = &snapshot.radices[3];
                    let width = [64, 32, 16, 8][self.word.get()];
                    for (i, b) in self.bits.borrow().iter().enumerate() {
                        let bit = 63 - i;
                        let value = binary
                            .as_bytes()
                            .iter()
                            .rev()
                            .nth(bit)
                            .map(|v| *v == b'1')
                            .unwrap_or(false);
                        b.set_content(Some(Rc::new(format!(
                            "{}\n{bit}",
                            if value { 1 } else { 0 }
                        ))));
                        b.set_is_enabled(bit < width && !snapshot.error);
                    }
                }
                self.refresh_history(&snapshot, mode);
            }
            Page::Converter(_) | Page::Currency => {
                view.get_control::<TextBlock>("Value1")
                    .set_text(Some(&snapshot.converter.from));
                view.get_control::<TextBlock>("Value2")
                    .set_text(Some(&snapshot.converter.to));
                view.get_control::<TextBlock>("SupplementaryResults")
                    .set_text(Some(&snapshot.converter.suggested.join(" · ")));
                for (name, id) in [
                    ("Unit1", snapshot.converter.from_id),
                    ("Unit2", snapshot.converter.to_id),
                ] {
                    let index = snapshot
                        .converter
                        .units
                        .iter()
                        .position(|u| u.id == id)
                        .unwrap_or(0);
                    view.get_control::<ComboBox>(name)
                        .set_selected_index(index as i32);
                }
            }
            Page::Date => self.refresh_date(&view),
            _ => {}
        }
    }
}
