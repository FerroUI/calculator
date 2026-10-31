//! Code-behind adaptation for the original Calculator history view.
use super::main_page::*;

impl MainPage {
    pub(super) fn refresh_history(self: &Rc<Self>, snapshot: &Snapshot, mode: Mode) {
        let panel = self.root.get_control::<StackPanel>("HistoryItems");
        panel.children().clear();
        if self.memory_tab.get() {
            if snapshot.memory.is_empty() {
                panel
                    .children()
                    .add(label("There's nothing saved in memory", 14.0));
            }
            for (index, value) in snapshot.memory.iter().enumerate() {
                let row = StackPanel::new();
                let b = button(value);
                b.set_horizontal_content_alignment(HorizontalAlignment::Right);
                let weak = Rc::downgrade(self);
                b.click(move |_, _| {
                    if let Some(this) = weak.upgrade() {
                        this.apply(|e| e.recall_memory(index));
                    }
                });
                row.children().add(b);
                let actions = StackPanel::new();
                actions.set_orientation(ferroui::base::layout::Orientation::Horizontal);
                for action in ["MC", "M+", "M−"] {
                    let b = button(action);
                    let weak = Rc::downgrade(self);
                    b.click(move |_, _| {
                        if let Some(this) = weak.upgrade() {
                            this.apply(|e| match action {
                                "MC" => e.remove_memory(index),
                                "M+" => e.add_memory(index),
                                _ => e.subtract_memory(index),
                            });
                        }
                    });
                    actions.children().add(b);
                }
                row.children().add(actions);
                panel.children().add(row);
            }
        } else {
            if snapshot.history.is_empty() {
                panel.children().add(label(
                    if mode == Mode::Programmer {
                        "History is unavailable in Programmer mode"
                    } else {
                        "There's no history yet"
                    },
                    14.0,
                ));
            }
            for (index, item) in snapshot.history.iter().enumerate().rev() {
                let row = StackPanel::new();
                let b = button("");
                let content = StackPanel::new();
                content.set_horizontal_alignment(HorizontalAlignment::Right);
                let expression = label(&item.expression, 14.0);
                expression.classes().add("historyExpression");
                let result = label(&item.result, 28.0);
                result.classes().add("historyResult");
                content.children().add(expression);
                content.children().add(result);
                b.set_content(Some(Control::boxed(content)));
                b.set_horizontal_content_alignment(HorizontalAlignment::Right);
                let weak = Rc::downgrade(self);
                b.click(move |_, _| {
                    if let Some(this) = weak.upgrade() {
                        this.apply(|e| e.recall_history(index));
                    }
                });
                row.children().add(b);
                let remove = button("Remove");
                let weak = Rc::downgrade(self);
                remove.click(move |_, _| {
                    if let Some(this) = weak.upgrade() {
                        this.apply(|e| e.remove_history(index));
                    }
                });
                row.children().add(remove);
                panel.children().add(row);
            }
        }
    }
}
