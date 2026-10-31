//! Gregorian counterpart of Calculator.ViewModels/Common/DateCalculator.cs.
//! The Windows calendar adapter is replaced; the native arithmetic engine is not involved.
use chrono::{Datelike, Days, Months, NaiveDate};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Difference {
    pub years: u32,
    pub months: u32,
    pub weeks: u32,
    pub days: u32,
    pub total_days: i64,
}

fn supported(date: NaiveDate) -> bool {
    (1601..=9999).contains(&date.year())
}
pub fn parse(text: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(text, "%Y-%m-%d")
        .ok()
        .filter(|d| supported(*d))
}
pub fn add_duration(
    date: NaiveDate,
    years: u32,
    months: u32,
    days: u64,
    subtract: bool,
) -> Option<NaiveDate> {
    if !supported(date) {
        return None;
    }
    // Match upstream: add larger units first; subtract smaller units first.
    let date = if subtract {
        date.checked_sub_days(Days::new(days))?
            .checked_sub_months(Months::new(months))?
            .checked_sub_months(Months::new(years.checked_mul(12)?))?
    } else {
        date.checked_add_months(Months::new(years.checked_mul(12)?))?
            .checked_add_months(Months::new(months))?
            .checked_add_days(Days::new(days))?
    };
    supported(date).then_some(date)
}
pub fn difference(a: NaiveDate, b: NaiveDate) -> Option<Difference> {
    if !supported(a) || !supported(b) {
        return None;
    }
    let (start, end) = if a <= b { (a, b) } else { (b, a) };
    let total_days = (end - start).num_days();
    let mut years = (end.year() - start.year()) as u32;
    let mut pivot = add_duration(start, years, 0, 0, false)?;
    if pivot > end {
        years -= 1;
        pivot = add_duration(start, years, 0, 0, false)?;
    }
    let mut months = 0;
    for n in 1..12 {
        if let Some(next) = add_duration(pivot, 0, n, 0, false) {
            if next <= end {
                months = n;
            } else {
                break;
            }
        }
    }
    pivot = add_duration(pivot, 0, months, 0, false)?;
    let rest = (end - pivot).num_days() as u32;
    Some(Difference {
        years,
        months,
        weeks: rest / 7,
        days: rest % 7,
        total_days,
    })
}
