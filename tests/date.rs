use ferro_calculator::date_calculator::*;
#[test]
fn gregorian_leap_days_month_end_clamping_and_supported_range() {
    let leap = parse("2024-02-29").unwrap();
    assert_eq!(add_duration(leap, 1, 0, 0, false), parse("2025-02-28"));
    assert_eq!(
        add_duration(parse("2024-01-31").unwrap(), 0, 1, 0, false),
        Some(leap)
    );
    let d = difference(parse("2024-02-28").unwrap(), parse("2024-03-01").unwrap()).unwrap();
    assert_eq!(d.total_days, 2);
    assert!(parse("2023-02-29").is_none());
    assert!(parse("1600-12-31").is_none());
    assert!(add_duration(parse("1601-01-01").unwrap(), 0, 0, 1, true).is_none());
    assert!(add_duration(leap, u32::MAX, 0, 0, false).is_none());
}
#[test]
fn duration_order_matches_upstream_subtracts_days_before_months() {
    let date = parse("2024-03-31").unwrap();
    assert_eq!(add_duration(date, 0, 1, 1, true), parse("2024-02-29"));
    let d = difference(parse("2020-01-01").unwrap(), parse("2022-03-12").unwrap()).unwrap();
    assert_eq!((d.years, d.months, d.weeks, d.days), (2, 2, 1, 4));
}
