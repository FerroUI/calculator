#![cfg(feature = "headless")]
mod common;
use common::{capture, region_pixels, settle};
use ferro_calculator::{
    app::App,
    engine::{Command, Mode},
    views::{MainPage, Page},
};
use ferroui::headless::HeadlessWindowExtensions;
use ferroui::{
    base::media::imaging::BitmapEncoderOptions,
    controls::{Border, CalendarDatePicker, ComboBox, NavigationView, TextBox},
    headless::{FerroHeadlessPlatformExtensions, FerroHeadlessPlatformOptions},
    prelude::*,
    skia::SkiaApplicationExtensions,
};
fn click(root: &Ref<ferroui::controls::UserControl>, name: &str) {
    root.get_control::<Button>(name).raise_event(
        &ferroui::base::interactivity::RoutedEventArgs::with_event(Button::click_event()),
    );
}
#[test]
fn xaml_commands_modes_flyouts_dates_converters_themes_and_layout() {
    AppBuilder::configure::<App>()
        .use_headless(FerroHeadlessPlatformOptions {
            use_headless_drawing: false,
            ..Default::default()
        })
        .use_skia()
        .setup_without_starting();
    let view = MainPage::new().unwrap();
    let window = Window::new();
    window.set_width(420.0);
    window.set_height(680.0);
    window.set_content(Some(Control::boxed(view.root.clone())));
    view.attach(&window);
    window.show();
    settle(&window);
    let navigation = view.root.get_control::<NavigationView>("NavView");
    navigation.set_is_pane_open(true);
    capture(&window, "navigation-light");
    // Select through the real NavigationView, including its footer Settings item.
    let scientific = navigation.menu_items().get_at(2);
    navigation.set_selected_item(scientific);
    assert_eq!(view.page.get(), Page::Calculator(Mode::Scientific));
    assert!(!navigation.is_pane_open());
    navigation.set_selected_item(Some(Control::boxed(navigation.settings_item())));
    assert_eq!(view.page.get(), Page::Settings);
    view.navigate(Page::Calculator(Mode::Standard));
    let ops = view.operators.borrow().clone().unwrap();
    click(&ops, "PlusButton");
    view.command(Command::Seven);
    click(&ops, "EqualButton");
    assert_eq!(view.engine.borrow().snapshot().unwrap().display, "7");
    capture(&window, "standard-light");
    MainPage::toggle_theme();
    capture(&window, "standard-dark");
    let number_pad = Control::from_boxed(
        &ops.get_control::<ferroui::controls::ContentControl>("NumberPad")
            .content()
            .unwrap(),
    )
    .unwrap()
    .cast::<ferroui::controls::UserControl>()
    .unwrap();
    verify_reveal(&window, &ops, "PlusButton");
    verify_reveal(&window, &number_pad, "Num7Button");
    MainPage::toggle_theme();
    verify_reveal(&window, &ops, "PlusButton");
    verify_reveal(&window, &number_pad, "Num7Button");
    // The generated source error groups must run through VSM, not only through
    // the Rust adapter's button disabling logic.
    view.command(Command::Clear);
    view.command(Command::One);
    view.command(Command::Divide);
    view.command(Command::Zero);
    view.command(Command::Equals);
    assert!(view.engine.borrow().snapshot().unwrap().error);
    let groups = ferroui::controls::visual_state::VisualStateManager::get_visual_state_groups(
        &ops.get_control::<ferroui::controls::Grid>("Root"),
    );
    assert_eq!(
        groups
            .get(0)
            .current_state()
            .map(|state| state.name())
            .as_deref(),
        Some("ErrorLayout")
    );
    assert!(!ops.get_control::<Button>("PlusButton").is_enabled());
    view.command(Command::Clear);
    assert_eq!(
        groups
            .get(0)
            .current_state()
            .map(|state| state.name())
            .as_deref(),
        Some("NoErrorLayout")
    );
    assert!(ops.get_control::<Button>("PlusButton").is_enabled());
    // Enter must evaluate even while an operator button has keyboard focus;
    // it must not activate that focused button again.
    view.command(Command::Clear);
    ops.get_control::<Button>("PlusButton").focus();
    window.key_text_input("2");
    window.key_text_input("+");
    window.key_text_input("2");
    use ferroui::base::input::{Key, PhysicalKey, RawInputModifiers};
    window.key_press(
        Key::Enter,
        RawInputModifiers::NONE,
        PhysicalKey::Enter,
        None,
    );
    window.key_release(
        Key::Enter,
        RawInputModifiers::NONE,
        PhysicalKey::Enter,
        None,
    );
    assert_eq!(view.engine.borrow().snapshot().unwrap().display, "4");
    // Drive actual pointer hit testing through the retained keypad tree.
    let clear = ops.get_control::<Button>("ClearButton");
    settle(&window);
    let point = clear
        .translate_point(ferroui::base::Point::new(10.0, 10.0), &window)
        .unwrap();
    window.mouse_down(
        point,
        ferroui::base::input::MouseButton::Left,
        RawInputModifiers::NONE,
    );
    window.mouse_up(
        point,
        ferroui::base::input::MouseButton::Left,
        RawInputModifiers::NONE,
    );
    assert_eq!(view.engine.borrow().snapshot().unwrap().display, "0");
    view.navigate(Page::Calculator(Mode::Scientific));
    window.set_height(800.0);
    settle(&window);
    view.command(Command::Three);
    view.command(Command::Zero);
    let ops = view.operators.borrow().clone().unwrap();
    ops.get_control::<Button>("TrigButton").open_flyout();
    settle(&window);
    assert!(
        ops.get_control::<Button>("TrigButton")
            .flyout()
            .unwrap()
            .is_open()
    );
    click(&ops, "SinButton");
    assert_eq!(view.engine.borrow().snapshot().unwrap().display, "0.5");
    capture(&window, "scientific");
    view.navigate(Page::Calculator(Mode::Programmer));
    window.set_height(860.0);
    let page = view.content.borrow().clone().unwrap();
    verify_radix_rows(&window, &page);
    MainPage::toggle_theme();
    verify_radix_rows(&window, &page);
    page.get_control::<ferroui::controls::RadioButton>("HexButton")
        .set_is_checked(Some(true));
    let ops = view.operators.borrow().clone().unwrap();
    click(&ops, "FButton");
    click(&ops, "FButton");
    assert_eq!(view.engine.borrow().snapshot().unwrap().radices[1], "255");
    capture(&window, "programmer");
    for id in 1..=12 {
        view.navigate(Page::Converter(id));
        settle(&window);
        view.engine.borrow_mut().converter_input("123").unwrap();
        view.refresh();
        capture(&window, &format!("converter-{id}"));
    }
    view.navigate(Page::Converter(4));
    let page = view.content.borrow().clone().unwrap();
    let state = view.engine.borrow().snapshot().unwrap().converter;
    let from = state.units.iter().position(|u| u.id == 46).unwrap();
    let to = state.units.iter().position(|u| u.id == 47).unwrap();
    page.get_control::<ComboBox>("Unit1")
        .set_selected_index(from as i32);
    page.get_control::<ComboBox>("Unit2")
        .set_selected_index(to as i32);
    view.engine.borrow_mut().converter_clear().unwrap();
    view.engine.borrow_mut().converter_input("100").unwrap();
    view.refresh();
    assert_eq!(
        page.get_control::<TextBlock>("Value2").text().as_deref(),
        Some("212")
    );
    capture(&window, "temperature");
    view.navigate(Page::Date);
    let page = view.content.borrow().clone().unwrap();
    use ferroui::base::utilities::DateTime;
    page.get_control::<CalendarDatePicker>("FromDate")
        .set_selected_date(Some(DateTime::new(2024, 2, 28)));
    page.get_control::<CalendarDatePicker>("ToDate")
        .set_selected_date(Some(DateTime::new(2024, 3, 1)));
    view.refresh();
    assert_eq!(
        page.get_control::<TextBlock>("TotalDays").text().as_deref(),
        Some("2 days")
    );
    page.get_control::<ComboBox>("DateCalculationOption")
        .set_selected_index(1);
    page.get_control::<TextBox>("Days").set_text(Some("1"));
    view.refresh();
    assert!(
        page.get_control::<TextBlock>("DateResult")
            .text()
            .unwrap()
            .contains("29")
    );
    capture(&window, "date");
    view.navigate(Page::Calculator(Mode::Standard));
    window.set_width(840.0);
    window.set_height(680.0);
    view.command(Command::One);
    view.command(Command::Add);
    view.command(Command::Two);
    view.command(Command::Equals);
    view.root
        .get_control::<Border>("HistoryPane")
        .set_is_visible(true);
    view.refresh();
    capture(&window, "history");
    for page in [Page::Graphing, Page::Currency, Page::Settings] {
        view.navigate(page);
        settle(&window);
        if page == Page::Currency {
            let state = view.engine.borrow().snapshot().unwrap().converter;
            assert_eq!(state.to, "50");
            assert!(
                state.suggested.is_empty(),
                "physical-unit suggestions must not leak into currency"
            );
            capture(&window, "currency-mock");
        }
    }
    window.close();
}

fn frame_bytes(window: &Ref<Window>) -> Vec<u8> {
    let bitmap = window.capture_rendered_frame().expect("Skia frame");
    let mut png = Vec::new();
    bitmap
        .save(&mut png, &BitmapEncoderOptions::Png(Default::default()))
        .unwrap();
    png
}

fn verify_radix_rows(window: &Ref<Window>, page: &Ref<ferroui::controls::UserControl>) {
    use ferroui::base::{
        Point,
        input::{MouseButton, RawInputModifiers},
    };
    use ferroui::controls::{RadioButton, presenters::ContentPresenter};
    settle(window);
    let names = ["HexButton", "DecimalButton", "OctButton", "BinaryButton"];
    let rows = names.map(|name| page.get_control::<RadioButton>(name));
    assert_eq!(rows[1].is_checked(), Some(true));
    let width = rows[0].bounds().width;
    let value_x = |row: &Ref<RadioButton>| {
        row.get_visual_descendants()
            .find_map(|v| v.cast::<ContentPresenter>())
            .unwrap()
            .translate_point(Point::default(), window)
            .unwrap()
            .x
    };
    for row in &rows {
        assert_eq!(
            row.bounds().width,
            width,
            "all radix rows span the same width"
        );
        assert_eq!(value_x(row), value_x(&rows[0]), "radix values align");
    }
    let binary = &rows[3];
    // Click the VALUE area, well beyond the label: the entire source row selects.
    let point = binary
        .translate_point(Point::new(width - 20.0, 16.0), window)
        .unwrap();
    window.mouse_move(point, RawInputModifiers::NONE);
    let root = binary
        .get_visual_descendants()
        .find_map(|v| {
            v.cast::<Border>()
                .filter(|v| v.name().as_deref() == Some("ControlRoot"))
        })
        .unwrap();
    assert!(
        root.background().unwrap().as_solid_color_brush().is_some(),
        "radix hover uses the original subtle fill, not keypad RevealBackgroundBrush"
    );
    window.mouse_down(point, MouseButton::Left, RawInputModifiers::NONE);
    window.mouse_up(point, MouseButton::Left, RawInputModifiers::NONE);
    assert_eq!(binary.is_checked(), Some(true));
    assert_eq!(rows[1].is_checked(), Some(false));
    let indicator = binary
        .get_visual_descendants()
        .find_map(|v| {
            v.cast::<Border>()
                .filter(|v| v.name().as_deref() == Some("CheckedRectangle"))
        })
        .unwrap();
    assert!(
        indicator.is_visible(),
        "selected radix has its source accent marker"
    );
    capture(
        window,
        if Application::current().unwrap().actual_theme_variant()
            == Some(ferroui::base::styling::ThemeVariant::dark())
        {
            "programmer-radix-hover-dark"
        } else {
            "programmer-radix-hover-light"
        },
    );
    rows[1].focus();
    window.key_press(
        ferroui::base::input::Key::Space,
        RawInputModifiers::NONE,
        ferroui::base::input::PhysicalKey::Space,
        None,
    );
    window.key_release(
        ferroui::base::input::Key::Space,
        RawInputModifiers::NONE,
        ferroui::base::input::PhysicalKey::Space,
        None,
    );
    assert_eq!(
        rows[1].is_checked(),
        Some(true),
        "keyboard selection restores Decimal"
    );
    assert_eq!(binary.is_checked(), Some(false));
    window.mouse_move(Point::new(-10.0, -10.0), RawInputModifiers::NONE);
}

fn verify_reveal(window: &Ref<Window>, ops: &Ref<ferroui::controls::UserControl>, name: &str) {
    use ferroui::base::media::MaterialPolicy;
    use ferroui::base::{
        Point,
        input::{MouseButton, RawInputModifiers},
    };
    use ferroui::controls::media::{
        RevealBackgroundBrush, RevealBrush, RevealBrushState, RevealHoverLight,
    };
    settle(window);
    let key = ops.get_control::<Button>(name);
    assert_eq!(key.font_weight(), ferroui::base::media::FontWeight::Normal);
    let dark = Application::current().unwrap().actual_theme_variant()
        == Some(ferroui::base::styling::ThemeVariant::dark());
    let foreground = key.foreground().expect("source text brush alias");
    let color = foreground
        .as_solid_color_brush()
        .expect("solid text")
        .color();
    assert_eq!(
        color.r,
        if dark { 255 } else { 0 },
        "key text must follow the theme"
    );
    let root = key
        .get_visual_descendants()
        .into_iter()
        .find_map(|v| {
            v.cast::<Border>()
                .filter(|v| v.name().as_deref() == Some("RootGrid"))
        })
        .expect("original button template root");
    assert_eq!(
        root.corner_radius(),
        ferroui::base::CornerRadius::uniform(4.0)
    );
    let first = key.translate_point(Point::new(8.0, 8.0), window).unwrap();
    let second = key
        .translate_point(
            Point::new(key.bounds().width - 8.0, key.bounds().height - 8.0),
            window,
        )
        .unwrap();
    window.mouse_move(first, RawInputModifiers::NONE);
    settle(window);
    assert_eq!(RevealBrush::get_state(&root), RevealBrushState::PointerOver);
    assert!(
        root.lights()
            .to_vec()
            .iter()
            .any(|light| light.is::<RevealHoverLight>())
    );
    // Compare two positions inside the SAME key: a solid hover fallback would
    // produce identical frames. Allow the border-light transition to finish.
    std::thread::sleep(std::time::Duration::from_millis(300));
    let left = frame_bytes(window);
    let origin = key.translate_point(Point::default(), window).unwrap();
    let interior = ferroui::base::PixelRect::new(origin.x as i32 + 8, origin.y as i32 + 12, 4, 4);
    let left_interior = region_pixels(window, interior);
    window.mouse_move(second, RawInputModifiers::NONE);
    let right = frame_bytes(window);
    assert!(left != right, "Reveal must follow the pointer within a key");
    assert_ne!(
        left_interior,
        region_pixels(window, interior),
        "hovered key surface must respond to the pointer, independently of border lighting"
    );
    capture(
        window,
        if Application::current().unwrap().actual_theme_variant()
            == Some(ferroui::base::styling::ThemeVariant::dark())
        {
            "reveal-dark"
        } else {
            "reveal-light"
        },
    );
    window.mouse_down(second, MouseButton::Left, RawInputModifiers::NONE);
    assert_eq!(RevealBrush::get_state(&root), RevealBrushState::Pressed);
    let pressed_background = root.background().unwrap();
    let pressed = pressed_background
        .as_object()
        .unwrap()
        .downcast_ref::<RevealBackgroundBrush>()
        .unwrap();
    assert_eq!(
        pressed.color(),
        if dark {
            ferroui::base::media::Color::from_argb(0x30, 255, 255, 255)
        } else {
            ferroui::base::media::Color::from_argb(0x30, 0, 0, 0)
        },
        "original pressed brush colour"
    );
    window.mouse_up(second, MouseButton::Left, RawInputModifiers::NONE);
    let background = root
        .background()
        .expect("Reveal background on original RootGrid");
    let reveal = background
        .as_object()
        .and_then(|b| b.downcast_ref::<RevealBackgroundBrush>())
        .expect("native RevealBackgroundBrush, not a solid substitute");
    assert_eq!(
        reveal.color(),
        if dark {
            ferroui::base::media::Color::from_argb(0x18, 255, 255, 255)
        } else {
            ferroui::base::media::Color::from_argb(0x17, 0, 0, 0)
        },
        "original hover brush colour"
    );
    assert!(reveal.is_connected());
    assert!(!reveal.is_in_fallback_mode());
    MaterialPolicy::set_simulate_disabled_by_policy(true);
    assert!(
        reveal.is_in_fallback_mode(),
        "respect reduced-effects policy"
    );
    std::thread::sleep(std::time::Duration::from_millis(350));
    let fallback = frame_bytes(window);
    window.mouse_move(first, RawInputModifiers::NONE);
    assert!(
        fallback == frame_bytes(window),
        "fallback must not contain pointer lighting"
    );
    capture(
        window,
        if dark {
            "fallback-dark"
        } else {
            "fallback-light"
        },
    );
    MaterialPolicy::set_simulate_disabled_by_policy(false);
    assert!(!reveal.is_in_fallback_mode());
    window.mouse_move(Point::new(-10.0, -10.0), RawInputModifiers::NONE);
    assert_eq!(RevealBrush::get_state(&root), RevealBrushState::Normal);
}
