//! Code-behind adaptation for the original Calculator input view.
use super::main_page::*;

impl MainPage {
    pub fn attach(self: &Rc<Self>, window: &Ref<Window>) {
        let this = self.clone();
        window.closed(move || {
            let _keep_alive = &this;
        });
        let weak = Rc::downgrade(self);
        let weak_window = window.downgrade();
        window.add_handler_with(
            InputElement::key_down_event(),
            move |_, event| {
                if let Some(this) = weak.upgrade() {
                    if this
                        .root
                        .get_control::<NavigationView>("NavView")
                        .is_pane_open()
                    {
                        if event.key == Key::Escape {
                            this.toggle_navigation();
                            event.set_handled(true);
                        }
                        return;
                    }
                    if matches!(
                        this.page.get(),
                        Page::Date | Page::Settings | Page::Graphing
                    ) {
                        return;
                    }
                    let modified = event
                        .key_modifiers
                        .intersects(KeyModifiers::CONTROL | KeyModifiers::META);
                    if modified && matches!(event.key, Key::C | Key::V) {
                        if let Some(clipboard) = weak_window.upgrade().and_then(|w| w.clipboard()) {
                            let weak = Rc::downgrade(&this);
                            let copy = event.key == Key::C;
                            ferroui::base::threading::Dispatcher::ui_thread()
                                .invoke_async_task_local(move || async move {
                                    let Some(this) = weak.upgrade() else { return };
                                    if copy {
                                        let state = this.engine.borrow().snapshot();
                                        if let Ok(state) = state {
                                            let _ = clipboard
                                                .set_text_async(Some(
                                                    if matches!(
                                                        this.page.get(),
                                                        Page::Calculator(_)
                                                    ) {
                                                        &state.display
                                                    } else {
                                                        &state.converter.from
                                                    },
                                                ))
                                                .await;
                                        }
                                    } else if let Ok(Some(text)) =
                                        clipboard.try_get_text_async().await
                                    {
                                        if matches!(this.page.get(), Page::Calculator(_)) {
                                            this.apply(|e| e.paste_number(&text));
                                        } else {
                                            this.apply(|e| {
                                                e.converter_clear()?;
                                                e.converter_input(text.trim())
                                            });
                                        }
                                    }
                                });
                        }
                        event.set_handled(true);
                        return;
                    }
                    let shift = event.key_modifiers.contains(KeyModifiers::SHIFT);
                    let kind = match (modified, shift) {
                        (true, true) => "VirtualKeyControlShiftChord",
                        (true, false) => "VirtualKeyControlChord",
                        (false, true) => "VirtualKeyShiftChord",
                        (false, false) => "VirtualKey",
                    };
                    let key = event.key.to_string();
                    let shortcut = if modified || shift || key.len() > 1 {
                        this.shortcut(kind, &key)
                    } else {
                        None
                    };
                    let command = shortcut.or(match event.key {
                        Key::Enter => Some(Command::Equals),
                        Key::Escape => Some(Command::Clear),
                        Key::Delete => Some(Command::ClearEntry),
                        Key::Back => Some(Command::Backspace),
                        _ => None,
                    });
                    if let Some(command) = command {
                        if matches!(this.page.get(), Page::Converter(_) | Page::Currency) {
                            match command {
                                Command::Clear | Command::ClearEntry => {
                                    this.apply(|e| e.converter_clear())
                                }
                                Command::Backspace => this.apply(|e| e.converter_backspace()),
                                _ => return,
                            }
                        } else {
                            this.command(command);
                        }
                        event.set_handled(true);
                    }
                    if event
                        .key_modifiers
                        .intersects(KeyModifiers::CONTROL | KeyModifiers::META)
                        && event.key == Key::L
                    {
                        this.toggle_navigation();
                        event.set_handled(true);
                    }
                }
            },
            ferroui::base::interactivity::RoutingStrategies::TUNNEL,
            false,
        );
        let weak = Rc::downgrade(self);
        window.add_handler(
            InputElement::text_input_event(),
            move |_, event: &TextInputEventArgs| {
                if let Some(this) = weak.upgrade() {
                    let Some(text) = event.text.as_ref() else {
                        return;
                    };
                    if matches!(this.page.get(), Page::Calculator(_)) {
                        if this
                            .root
                            .get_control::<NavigationView>("NavView")
                            .is_pane_open()
                        {
                            return;
                        }
                        if let Some(command) = this.shortcut("Character", text).or_else(|| {
                            if text.len() == 1 {
                                this.shortcut("VirtualKey", &text.to_ascii_uppercase())
                            } else {
                                None
                            }
                        }) {
                            this.command(command);
                        } else {
                            this.apply(|e| e.enter(text));
                        }
                        event.set_handled(true);
                    } else if matches!(this.page.get(), Page::Converter(_) | Page::Currency) {
                        this.apply(|e| e.converter_input(text));
                        event.set_handled(true);
                    }
                }
            },
        );
    }
    pub(super) fn shortcut(&self, kind: &str, key: &str) -> Option<Command> {
        let Page::Calculator(mode) = self.page.get() else {
            return None;
        };
        let file = match mode {
            Mode::Standard => "CalculatorStandardOperators.xaml",
            Mode::Scientific => "CalculatorScientificOperators.xaml",
            Mode::Programmer => "CalculatorProgrammerRadixOperators.xaml",
        };
        for (file, root) in [
            (file, self.operators.borrow().clone()),
            ("NumberPad.xaml", self.number_pad.borrow().clone()),
        ] {
            if let Some(root) = root {
                for binding in &bindings()[file] {
                    if binding.shortcuts.get(kind).is_some_and(|s| s == key)
                        && root
                            .find_control::<Button>(&binding.name)
                            .is_some_and(|b| b.is_enabled())
                    {
                        return Command::from_name(&binding.command);
                    }
                }
            }
        }
        None
    }
    pub(super) fn toggle_navigation(&self) {
        let navigation = self.root.get_control::<NavigationView>("NavView");
        navigation.set_is_pane_open(!navigation.is_pane_open());
    }
}
