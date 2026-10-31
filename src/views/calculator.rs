//! Code-behind adaptation for the original Calculator calculator view.
use super::main_page::*;

impl MainPage {
    pub(super) fn calculator(self: &Rc<Self>, mode: Mode) -> Ref<UserControl> {
        let view: Ref<UserControl> = load(include_str!("Calculator.xaml"));
        let root: Ref<Control> = view.clone().upcast();
        let file = match mode {
            Mode::Standard => "CalculatorStandardOperators.xaml",
            Mode::Scientific => "CalculatorScientificOperators.xaml",
            Mode::Programmer => "CalculatorProgrammerRadixOperators.xaml",
        };
        let operators: Ref<UserControl> = load(match mode {
            Mode::Standard => include_str!("CalculatorStandardOperators.xaml"),
            Mode::Scientific => include_str!("CalculatorScientificOperators.xaml"),
            Mode::Programmer => include_str!("CalculatorProgrammerRadixOperators.xaml"),
        });
        self.wire_keypad(&operators, file, false);
        let number_pad: Ref<UserControl> = load(include_str!("NumberPad.xaml"));
        self.wire_keypad(&number_pad, "NumberPad.xaml", false);
        operators
            .get_control::<ContentControl>("NumberPad")
            .set_content(Some(Control::boxed(number_pad.clone())));
        view.get_control::<ContentControl>("OperatorsPanel")
            .set_content(Some(Control::boxed(operators.clone())));
        *self.number_pad.borrow_mut() = Some(number_pad);
        *self.operators.borrow_mut() = Some(operators.clone());
        if mode == Mode::Scientific {
            let angle: Ref<UserControl> =
                load(include_str!("CalculatorScientificAngleButtons.xaml"));
            view.get_control::<ContentControl>("ScientificAngleHost")
                .set_content(Some(Control::boxed(angle.clone())));
            self.bind(&angle.clone().upcast(), "DegreeButton", |this| {
                let next = (this.angle.get() + 1) % 3;
                this.angle.set(next);
                this.command([Command::Degree, Command::Radians, Command::Grads][next]);
            });
            if let Some(b) =
                angle.find_control::<ferroui::controls::primitives::ToggleButton>("FtoeButton")
            {
                let weak = Rc::downgrade(self);
                b.click(move |_, _| {
                    if let Some(this) = weak.upgrade() {
                        this.command(Command::FToE);
                    }
                });
            }
            *self.angles.borrow_mut() = Some(angle);
            let weak = Rc::downgrade(self);
            let ops = operators.downgrade();
            operators
                .get_control::<ferroui::controls::primitives::ToggleButton>("ShiftButton")
                .click(move |b, _| {
                    let Some(ops) = ops.upgrade() else { return };
                    let checked = b
                        .downcast_ref::<ferroui::controls::primitives::ToggleButton>()
                        .and_then(|b| b.is_checked())
                        .unwrap_or(false);
                    ops.get_control::<Grid>("Row1").set_is_visible(!checked);
                    ops.get_control::<Grid>("InvRow1").set_is_visible(checked);
                    if let Some(this) = weak.upgrade() {
                        this.refresh();
                    }
                });
            for toggle in ["TrigShiftButton", "HypButton"] {
                let ops = operators.downgrade();
                if let Some(b) =
                    operators.find_control::<ferroui::controls::primitives::ToggleButton>(toggle)
                {
                    b.click(move |_, _| {
                        let Some(ops) = ops.upgrade() else { return };
                        let inverse = ops
                            .get_control::<ferroui::controls::primitives::ToggleButton>(
                                "TrigShiftButton",
                            )
                            .is_checked()
                            == Some(true);
                        let hyp = ops
                            .get_control::<ferroui::controls::primitives::ToggleButton>("HypButton")
                            .is_checked()
                            == Some(true);
                        for (name, visible) in [
                            ("TrigFunctions", !inverse && !hyp),
                            ("InverseTrigFunctions", inverse && !hyp),
                            ("HyperbolicTrigFunctions", !inverse && hyp),
                            ("InverseHyperbolicTrigFunctions", inverse && hyp),
                        ] {
                            ops.get_control::<Grid>(name).set_is_visible(visible);
                        }
                    });
                }
            }
        }
        for (name, command) in [
            ("MemoryClear", Command::MemoryClear),
            ("MemoryRecall", Command::MemoryRecall),
            ("MemoryAdd", Command::MemoryAdd),
            ("MemorySubtract", Command::MemorySubtract),
            ("MemoryStore", Command::MemoryStore),
        ] {
            self.bind(&root, name, move |this| match command {
                Command::MemoryClear => this.apply(|e| e.clear_memory()),
                Command::MemoryRecall => this.apply(|e| e.recall_memory(0)),
                Command::MemoryAdd => this.apply(|e| {
                    if e.snapshot()?.memory.is_empty() {
                        e.store_memory()
                    } else {
                        e.add_memory(0)
                    }
                }),
                Command::MemorySubtract => this.apply(|e| e.subtract_memory(0)),
                _ => this.command(command),
            });
        }
        self.bind(&root, "MemoryShow", |this| {
            this.memory_tab.set(true);
            this.root
                .get_control::<Border>("HistoryPane")
                .set_is_visible(true);
            this.refresh();
        });
        view.get_control::<StackPanel>("ProgrammerPanel")
            .set_is_visible(mode == Mode::Programmer);
        if mode == Mode::Programmer {
            for (name, command, radix) in [
                ("HexButton", Command::HexButton, 16),
                ("DecimalButton", Command::DecButton, 10),
                ("OctButton", Command::OctButton, 8),
                ("BinaryButton", Command::BinButton, 2),
            ] {
                let weak = Rc::downgrade(self);
                view.get_control::<ferroui::controls::RadioButton>(name)
                    .is_checked_changed(move |button, _| {
                        let Some(this) = weak.upgrade() else { return };
                        let checked = button
                            .downcast_ref::<ferroui::controls::RadioButton>()
                            .is_some_and(|b| b.is_checked() == Some(true));
                        if checked && this.radix.get() != radix {
                            this.radix.set(radix);
                            this.command(command);
                        }
                    });
            }
            self.bind(&root, "WordSizeButton", |this| {
                let n = (this.word.get() + 1) % 4;
                this.word.set(n);
                this.command([Command::Qword, Command::Dword, Command::Word, Command::Byte][n]);
            });
            self.bind(&root, "BitFlip", |this| {
                if let Some(v) = this.content.borrow().as_ref() {
                    let panel = v.get_control::<ferroui::controls::ScrollViewer>("BitFlipPanel");
                    let show = !panel.is_visible();
                    panel.set_is_visible(show);
                    v.get_control::<ContentControl>("OperatorsPanel")
                        .set_is_visible(!show);
                }
            });
            for shift in [
                "ArithmeticShiftButton",
                "LogicalShiftButton",
                "RotateCircularButton",
                "RotateCarryShiftButton",
            ] {
                let ops = operators.downgrade();
                self.bind(&operators.clone().upcast(), shift, move |_| {
                    let Some(ops) = ops.upgrade() else { return };
                    for (name, visible) in [
                        ("LshButton", shift == "ArithmeticShiftButton"),
                        ("RshButton", shift == "ArithmeticShiftButton"),
                        ("LshLogicalButton", shift == "LogicalShiftButton"),
                        ("RshLogicalButton", shift == "LogicalShiftButton"),
                        ("RolButton", shift == "RotateCircularButton"),
                        ("RorButton", shift == "RotateCircularButton"),
                        ("RolCarryButton", shift == "RotateCarryShiftButton"),
                        ("RorCarryButton", shift == "RotateCarryShiftButton"),
                    ] {
                        ops.get_control::<Button>(name).set_is_visible(visible);
                    }
                });
            }
            let bits = view.get_control::<UniformGrid>("BitButtons");
            for bit in (0..64).rev() {
                let b = button(&format!("0\n{bit}"));
                let weak = Rc::downgrade(self);
                b.click(move |_, _| {
                    if let Some(this) = weak.upgrade() {
                        this.apply(|e| e.toggle_bit(bit));
                    }
                });
                bits.children().add(b.clone());
                self.bits.borrow_mut().push(b);
            }
        }
        view
    }
    pub(super) fn wire_keypad(
        self: &Rc<Self>,
        view: &Ref<UserControl>,
        file: &str,
        converter: bool,
    ) {
        let metadata = bindings();
        for binding in &metadata[file] {
            let command = Command::from_name(&binding.command).expect("native command");
            let b = view.get_control::<Button>(&binding.name);
            let weak = Rc::downgrade(self);
            let view = view.downgrade();
            b.click(move |_, _| {
                if let (Some(this), Some(view)) = (weak.upgrade(), view.upgrade()) {
                    if converter {
                        this.converter_command(command);
                    } else {
                        this.command(command);
                    }
                    for name in [
                        "TrigButton",
                        "FuncButton",
                        "BitwiseButton",
                        "BitShiftButton",
                    ] {
                        if let Some(b) = view.find_control::<Button>(name) {
                            b.close_flyout();
                        }
                    }
                }
            });
        }
    }
}

impl MainPage {
    pub(super) fn update_keypad(&self, root: &Ref<UserControl>, snapshot: &Snapshot, mode: Mode) {
        // Drive the original ErrorVisualStates through FerroUI's native VSM.
        // Radix-dependent digits and custom properties still use the adapter.
        ferroui::controls::visual_state::VisualStateManager::go_to_state(
            root,
            if snapshot.error {
                "ErrorLayout"
            } else {
                "NoErrorLayout"
            },
            false,
        );
        let metadata = bindings();
        let file = if root.find_control::<Button>("Num0Button").is_some() {
            "NumberPad.xaml"
        } else {
            match mode {
                Mode::Standard => "CalculatorStandardOperators.xaml",
                Mode::Scientific => "CalculatorScientificOperators.xaml",
                Mode::Programmer => "CalculatorProgrammerRadixOperators.xaml",
            }
        };
        for binding in &metadata[file] {
            if let Some(b) = root.find_control::<Button>(&binding.name) {
                let cmd = Command::from_name(&binding.command).expect("command");
                let digit = cmd.id() - 130;
                let enabled = if (0..=15).contains(&digit) {
                    mode != Mode::Programmer || (digit as u32) < self.radix.get()
                } else if cmd == Command::Decimal {
                    !snapshot.error && mode != Mode::Programmer
                } else {
                    !snapshot.error
                        || matches!(
                            cmd,
                            Command::Clear | Command::ClearEntry | Command::Backspace
                        )
                };
                b.set_is_enabled(enabled);
            }
        }
        if let (Some(clear), Some(ce)) = (
            root.find_control::<Button>("ClearButton"),
            root.find_control::<Button>("ClearEntryButton"),
        ) && mode != Mode::Standard
        {
            let empty = snapshot.display == "0";
            clear.set_is_visible(empty);
            ce.set_is_visible(!empty);
        }
    }
}
