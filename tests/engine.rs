use ferro_calculator::engine::{Command as C, Engine, Mode};
// Ratpack has process-wide constants. Each scenario owns one engine; serialize test scenarios.
static SCENARIO: std::sync::Mutex<()> = std::sync::Mutex::new(());
fn value(e: &Engine) -> String {
    e.snapshot().unwrap().display.replace(',', "")
}

#[test]
fn original_standard_engine_precision_chaining_repeated_equals_and_errors() {
    let _guard = SCENARIO.lock().unwrap_or_else(|e| e.into_inner());
    let mut e = Engine::new().unwrap();
    e.enter("0.1+0.2=").unwrap();
    assert_eq!(value(&e), "0.3");
    e.send(C::Clear).unwrap();
    e.enter("2+3*4=").unwrap();
    assert_eq!(value(&e), "20");
    e.send(C::Clear).unwrap();
    e.enter("9007199254740993+1=").unwrap();
    assert_eq!(value(&e), "9007199254740994");
    e.send(C::Clear).unwrap();
    e.enter("5+2==").unwrap();
    assert_eq!(value(&e), "9");
    e.send(C::Clear).unwrap();
    e.enter("1/0=").unwrap();
    assert!(e.snapshot().unwrap().error);
    assert!(value(&e).to_lowercase().contains("zero"));
    e.send(C::Clear).unwrap();
    e.enter("9").unwrap();
    e.send(C::Sqrt).unwrap();
    assert_eq!(value(&e), "3");
    let before = value(&e);
    assert!(e.enter("42oops").is_err());
    assert_eq!(value(&e), before);
}

#[test]
fn scientific_precedence_functions_angles_parentheses_and_factorial() {
    let _guard = SCENARIO.lock().unwrap_or_else(|e| e.into_inner());
    let mut e = Engine::new().unwrap();
    e.set_mode(Mode::Scientific).unwrap();
    e.enter("2+3*4=").unwrap();
    assert_eq!(value(&e), "14");
    e.send(C::Clear).unwrap();
    e.enter("(2+3)*4=").unwrap();
    assert_eq!(value(&e), "20");
    e.send(C::Clear).unwrap();
    e.enter("30").unwrap();
    e.send(C::Degree).unwrap();
    e.send(C::Sin).unwrap();
    assert_eq!(value(&e), "0.5");
    e.send(C::Clear).unwrap();
    e.enter("25").unwrap();
    e.send(C::Factorial).unwrap();
    assert_eq!(value(&e), "15511210043330985984000000");
    e.send(C::Clear).unwrap();
    e.enter("2").unwrap();
    e.send(C::XPowerY).unwrap();
    e.enter("100=").unwrap();
    assert_eq!(value(&e), "1267650600228229401496703205376");
}

#[test]
fn programmer_radices_word_sizes_signed_overflow_and_bits() {
    let _guard = SCENARIO.lock().unwrap_or_else(|e| e.into_inner());
    let mut e = Engine::new().unwrap();
    e.set_mode(Mode::Programmer).unwrap();
    e.send(C::HexButton).unwrap();
    e.enter("FF").unwrap();
    assert_eq!(
        e.snapshot().unwrap().radices,
        ["FF", "255", "377", "11111111"]
    );
    e.send(C::Byte).unwrap();
    assert_eq!(e.snapshot().unwrap().radices[1], "-1");
    e.send(C::Clear).unwrap();
    e.send(C::DecButton).unwrap();
    e.enter("127+1=").unwrap();
    assert_eq!(value(&e), "-128");
    e.send(C::Clear).unwrap();
    e.toggle_bit(7).unwrap();
    assert_eq!(value(&e), "-128");
    e.toggle_bit(7).unwrap();
    assert_eq!(value(&e), "0");
    assert!(e.toggle_bit(64).is_err());
    e.send(C::Qword).unwrap();
    e.enter("12").unwrap();
    e.send(C::And).unwrap();
    e.enter("10=").unwrap();
    assert_eq!(value(&e), "8");
}

#[test]
fn native_history_and_multi_item_memory() {
    let _guard = SCENARIO.lock().unwrap_or_else(|e| e.into_inner());
    let mut e = Engine::new().unwrap();
    e.enter("12+3=").unwrap();
    assert_eq!(e.snapshot().unwrap().history[0].result, "15");
    e.store_memory().unwrap();
    e.send(C::Clear).unwrap();
    e.enter("7").unwrap();
    e.store_memory().unwrap();
    assert_eq!(e.snapshot().unwrap().memory, ["7", "15"]);
    e.recall_memory(1).unwrap();
    assert_eq!(value(&e), "15");
    e.add_memory(0).unwrap();
    assert_eq!(e.snapshot().unwrap().memory[0], "22");
    e.subtract_memory(0).unwrap();
    assert_eq!(e.snapshot().unwrap().memory[0], "7");
    assert!(e.recall_memory(usize::MAX).is_err());
    assert!(e.remove_memory(99).is_err());
    e.remove_memory(0).unwrap();
    assert_eq!(e.snapshot().unwrap().memory, ["15"]);
    e.recall_history(0).unwrap();
    assert_eq!(value(&e), "15");
    e.enter("+1=").unwrap();
    assert_eq!(value(&e), "16");
    e.clear_history().unwrap();
    assert!(e.snapshot().unwrap().history.is_empty());
    e.clear_memory().unwrap();
    assert!(e.snapshot().unwrap().memory.is_empty());
    e.send(C::Clear).unwrap();
    e.enter("9").unwrap();
    e.subtract_memory(0).unwrap();
    assert_eq!(e.snapshot().unwrap().memory, ["-9"]);
}

#[test]
fn native_converter_all_categories_temperature_offsets_and_bidirectional_input() {
    let _guard = SCENARIO.lock().unwrap_or_else(|e| e.into_inner());
    let mut e = Engine::new().unwrap();
    assert_eq!(e.snapshot().unwrap().converter.categories.len(), 13);
    for category in e.snapshot().unwrap().converter.categories {
        e.set_category(category.id).unwrap();
        let state = e.snapshot().unwrap().converter;
        assert!(!state.units.is_empty(), "{}", category.name);
        e.set_units(state.units[0].id, state.units[0].id).unwrap();
        e.converter_clear().unwrap();
        e.converter_input("123").unwrap();
        let state = e.snapshot().unwrap().converter;
        assert_eq!(state.from, state.to, "{}", category.name);
    }
    e.set_category(4).unwrap();
    e.set_units(46, 47).unwrap();
    e.converter_clear().unwrap();
    e.converter_input("100").unwrap();
    assert_eq!(e.snapshot().unwrap().converter.to, "212");
    e.converter_clear().unwrap();
    e.converter_input("-40").unwrap();
    assert_eq!(e.snapshot().unwrap().converter.to, "-40");
    e.set_category(2).unwrap();
    e.set_units(34, 32).unwrap();
    e.converter_clear().unwrap();
    e.converter_input("1").unwrap();
    assert!(e.snapshot().unwrap().converter.to.starts_with("39.370"));
    assert!(e.set_category(-1).is_err());
    assert!(e.set_units(9999, 32).is_err());
    e.set_category(0).unwrap();
    assert_eq!(
        e.snapshot().unwrap().converter.to,
        "1",
        "changing category must immediately recalculate using the saved unit selection"
    );
    e.set_units(1001, 1002).unwrap();
    e.converter_clear().unwrap();
    e.converter_input("10").unwrap();
    assert_eq!(e.snapshot().unwrap().converter.to, "5");
    e.converter_swap().unwrap();
    let state = e.snapshot().unwrap().converter;
    assert_eq!((state.from.as_str(), state.to.as_str()), ("5", "10"));
}

#[test]
fn clipboard_numbers_preserve_precision_and_validate_before_changing_state() {
    let _guard = SCENARIO.lock().unwrap_or_else(|e| e.into_inner());
    let mut e = Engine::new().unwrap();
    e.paste_number("9,007,199,254,740,993").unwrap();
    assert_eq!(value(&e), "9007199254740993");
    assert!(e.paste_number("1,2").is_err());
    assert_eq!(value(&e), "9007199254740993");
    e.set_mode(Mode::Scientific).unwrap();
    e.paste_number("-1.25e-3").unwrap();
    e.send(C::Equals).unwrap();
    assert_eq!(value(&e), "-0.00125");
    assert!(e.paste_number("2+3").is_err());
    assert_eq!(value(&e), "-0.00125");
    e.set_mode(Mode::Programmer).unwrap();
    e.send(C::HexButton).unwrap();
    e.paste_number("FACE").unwrap();
    assert_eq!(value(&e), "FACE");
    e.send(C::BinButton).unwrap();
    assert!(e.paste_number("102").is_err());
}

#[test]
fn history_replays_original_scientific_unary_and_parenthesis_commands() {
    let _guard = SCENARIO.lock().unwrap_or_else(|e| e.into_inner());
    let mut e = Engine::new().unwrap();
    e.set_mode(Mode::Scientific).unwrap();
    e.enter("9").unwrap();
    e.send(C::Sqrt).unwrap();
    e.enter("+2=").unwrap();
    assert_eq!(value(&e), "5");
    e.send(C::Clear).unwrap();
    e.recall_history(0).unwrap();
    assert_eq!(value(&e), "5");
    e.enter("+1=").unwrap();
    assert_eq!(value(&e), "6");
    e.send(C::Clear).unwrap();
    e.enter("(3+4)*2=").unwrap();
    let n = e.snapshot().unwrap().history.len();
    e.send(C::Clear).unwrap();
    e.recall_history(n - 1).unwrap();
    assert_eq!(value(&e), "14");
}
