//! Code-behind adaptation for the original Calculator unit converter view.
use super::main_page::*;

impl MainPage {
    pub(super) fn converter_command(self: &Rc<Self>, command: Command) {
        let text = match command {
            Command::Zero => "0",
            Command::One => "1",
            Command::Two => "2",
            Command::Three => "3",
            Command::Four => "4",
            Command::Five => "5",
            Command::Six => "6",
            Command::Seven => "7",
            Command::Eight => "8",
            Command::Nine => "9",
            Command::Decimal => ".",
            Command::Negate => "-",
            _ => return,
        };
        self.apply(|e| e.converter_input(text));
    }
    pub(super) fn converter(self: &Rc<Self>) -> Ref<UserControl> {
        let view: Ref<UserControl> = load(include_str!("UnitConverter.xaml"));
        let root: Ref<Control> = view.clone().upcast();
        view.get_control::<Button>("ConverterNegate")
            .set_is_visible(matches!(self.page.get(), Page::Converter(4 | 9 | 12)));
        let pad: Ref<UserControl> = load(include_str!("NumberPad.xaml"));
        self.wire_keypad(&pad, "NumberPad.xaml", true);
        view.get_control::<ContentControl>("NumberPad")
            .set_content(Some(Control::boxed(pad.clone())));
        *self.number_pad.borrow_mut() = Some(pad);
        let state = self
            .engine
            .borrow()
            .snapshot()
            .expect("native snapshot")
            .converter;
        for name in ["Unit1", "Unit2"] {
            let combo = view.get_control::<ComboBox>(name);
            for u in &state.units {
                combo.items().add(Some(Rc::new(u.name.clone())));
            }
            combo.set_selected_index(
                state
                    .units
                    .iter()
                    .position(|u| {
                        u.id == if name == "Unit1" {
                            state.from_id
                        } else {
                            state.to_id
                        }
                    })
                    .unwrap_or(0) as i32,
            );
            let weak = Rc::downgrade(self);
            let view = view.downgrade();
            combo.selection_changed(move |_, _| {
                if let (Some(this), Some(view)) = (weak.upgrade(), view.upgrade()) {
                    if this.updating.get() {
                        return;
                    }
                    let a = view.get_control::<ComboBox>("Unit1").selected_index();
                    let b = view.get_control::<ComboBox>("Unit2").selected_index();
                    let state = this
                        .engine
                        .borrow()
                        .snapshot()
                        .expect("native snapshot")
                        .converter;
                    if let (Some(from), Some(to)) =
                        (state.units.get(a as usize), state.units.get(b as usize))
                    {
                        this.apply(|e| e.set_units(from.id, to.id));
                    }
                }
            });
        }
        self.bind(&root, "SwapUnits", |this| {
            this.apply(|e| e.converter_swap())
        });
        self.bind(&root, "ClearEntryButton", |this| {
            this.apply(|e| e.converter_clear())
        });
        self.bind(&root, "BackSpaceButton", |this| {
            this.apply(|e| e.converter_backspace())
        });
        self.bind(&root, "ConverterNegate", |this| {
            this.converter_command(Command::Negate)
        });
        view
    }
}
