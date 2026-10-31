//! Code-behind adaptation for the original Calculator date calculator view.
use super::main_page::*;

impl MainPage {
    pub(super) fn date(self: &Rc<Self>) -> Ref<UserControl> {
        let view: Ref<UserControl> = load(include_str!("DateCalculator.xaml"));
        let today = chrono::Local::now().date_naive();
        for name in ["FromDate", "ToDate"] {
            let picker = view.get_control::<CalendarDatePicker>(name);
            picker.set_selected_date(Some(DateTime::new(
                today.year(),
                today.month() as i32,
                today.day() as i32,
            )));
            let weak = Rc::downgrade(self);
            picker.selected_date_changed(move |_| {
                if let Some(this) = weak.upgrade() {
                    this.refresh();
                }
            });
        }
        for name in ["DateCalculationOption", "AddSubtract"] {
            let weak = Rc::downgrade(self);
            view.get_control::<ComboBox>(name)
                .selection_changed(move |_, _| {
                    if let Some(this) = weak.upgrade() {
                        this.refresh();
                    }
                });
        }
        for name in ["Years", "Months", "Days"] {
            let weak = Rc::downgrade(self);
            view.get_control::<TextBox>(name).text_changed(move |_, _| {
                if let Some(this) = weak.upgrade() {
                    this.refresh();
                }
            });
        }
        view
    }
    pub(super) fn refresh_date(&self, view: &Ref<UserControl>) {
        let duration = view
            .get_control::<ComboBox>("DateCalculationOption")
            .selected_index()
            == 1;
        view.get_control::<StackPanel>("DurationPanel")
            .set_is_visible(duration);
        view.get_control::<StackPanel>("ToDatePanel")
            .set_is_visible(!duration);
        let date = |name| {
            view.get_control::<CalendarDatePicker>(name)
                .selected_date()
                .and_then(|d| {
                    chrono::NaiveDate::from_ymd_opt(d.year(), d.month() as u32, d.day() as u32)
                })
        };
        let result = (|| {
            let from = date("FromDate")?;
            if duration {
                let count = |name| {
                    view.get_control::<TextBox>(name)
                        .text()?
                        .parse::<u32>()
                        .ok()
                };
                let subtract = view.get_control::<ComboBox>("AddSubtract").selected_index() == 1;
                let d = date_calculator::add_duration(
                    from,
                    count("Years")?,
                    count("Months")?,
                    u64::from(count("Days")?),
                    subtract,
                )?;
                Some((d.format("%A, %B %-d, %Y").to_string(), String::new()))
            } else {
                let d = date_calculator::difference(from, date("ToDate")?)?;
                Some((
                    format!(
                        "{} years, {} months, {} weeks, {} days",
                        d.years, d.months, d.weeks, d.days
                    ),
                    format!("{} days", d.total_days),
                ))
            }
        })();
        let (primary, secondary) = result.unwrap_or_else(|| {
            (
                "Enter valid dates and nonnegative whole durations".into(),
                String::new(),
            )
        });
        view.get_control::<TextBlock>("DateResult")
            .set_text(Some(&primary));
        view.get_control::<TextBlock>("TotalDays")
            .set_text(Some(&secondary));
    }
}
