//! Safe, thread-affine ownership of the original C++ CalculatorManager.
//! Arithmetic and formatting are always performed by CalcManager/CEngine/Ratpack.
use serde::Deserialize;
use std::{
    ffi::{CStr, c_char, c_void},
    marker::PhantomData,
    ptr::NonNull,
    rc::Rc,
    sync::Mutex,
};
#[path = "engine_commands.rs"]
mod commands;
pub use commands::Command;

static ENGINE_LOCK: Mutex<()> = Mutex::new(());
unsafe extern "C" {
    fn ferro_calc_create() -> *mut c_void;
    fn ferro_calc_destroy(p: *mut c_void);
    fn ferro_calc_action(p: *mut c_void, action: i32, a: i32, b: i32) -> i32;
    fn ferro_calc_snapshot(p: *mut c_void) -> *const c_char;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineError(pub String);
impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for EngineError {}
pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct HistoryItem {
    pub expression: String,
    pub result: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct NamedItem {
    pub id: i32,
    pub name: String,
}
#[derive(Clone, Debug, Deserialize)]
pub struct ConverterState {
    pub from: String,
    pub to: String,
    pub from_id: i32,
    pub to_id: i32,
    pub categories: Vec<NamedItem>,
    pub units: Vec<NamedItem>,
    pub suggested: Vec<String>,
}
#[derive(Clone, Debug, Deserialize)]
pub struct Snapshot {
    pub display: String,
    pub expression: String,
    pub error: bool,
    pub parentheses: u32,
    pub memory: Vec<String>,
    pub history: Vec<HistoryItem>,
    pub radices: [String; 4],
    pub converter: ConverterState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Standard,
    Scientific,
    Programmer,
}
impl Mode {
    pub fn title(self) -> &'static str {
        match self {
            Self::Standard => "Standard",
            Self::Scientific => "Scientific",
            Self::Programmer => "Programmer",
        }
    }
}

pub struct Engine {
    handle: NonNull<c_void>,
    mode: Mode,
    radix: u32,
    // The retained UI and native engine are affine to their owning thread.
    _thread_affine: PhantomData<Rc<()>>,
}
impl Engine {
    pub fn new() -> Result<Self> {
        let _guard = ENGINE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: the C ABI returns either a fresh owned Context or null; exceptions are caught.
        let handle = NonNull::new(unsafe { ferro_calc_create() }).ok_or_else(|| {
            EngineError("The original calculation engine could not initialize".into())
        })?;
        Ok(Self {
            handle,
            mode: Mode::Standard,
            radix: 10,
            _thread_affine: PhantomData,
        })
    }
    fn action(&mut self, action: i32, a: i32, b: i32) -> Result<()> {
        let _guard = ENGINE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: self owns a live Context, all calls are serialized, arguments are checked below
        // and in the bridge. No callback retains Rust references.
        match unsafe { ferro_calc_action(self.handle.as_ptr(), action, a, b) } {
            0 => Ok(()),
            1 => Err(EngineError("Invalid engine action or item index".into())),
            _ => Err(EngineError(
                "The original calculation engine rejected the operation".into(),
            )),
        }
    }
    pub fn snapshot(&self) -> Result<Snapshot> {
        let _guard = ENGINE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: the returned NUL-terminated buffer belongs to our live Context and is copied
        // before any subsequent native operation can invalidate it.
        let ptr = unsafe { ferro_calc_snapshot(self.handle.as_ptr()) };
        if ptr.is_null() {
            return Err(EngineError("Could not read native engine state".into()));
        }
        let bytes = unsafe { CStr::from_ptr(ptr) }.to_bytes();
        serde_json::from_slice(bytes).map_err(|e| EngineError(e.to_string()))
    }
    pub fn send(&mut self, command: Command) -> Result<()> {
        if command == Command::MemoryStore {
            return self.store_memory();
        }
        self.action(0, command.id(), 0)?;
        match command {
            Command::IsStandardMode => {
                self.mode = Mode::Standard;
                self.radix = 10;
            }
            Command::IsScientificMode => {
                self.mode = Mode::Scientific;
                self.radix = 10;
            }
            Command::IsProgrammerMode => {
                self.mode = Mode::Programmer;
                self.radix = 10;
            }
            Command::HexButton => self.radix = 16,
            Command::DecButton => self.radix = 10,
            Command::OctButton => self.radix = 8,
            Command::BinButton => self.radix = 2,
            _ => {}
        }
        Ok(())
    }
    pub fn set_mode(&mut self, mode: Mode) -> Result<()> {
        self.send(match mode {
            Mode::Standard => Command::IsStandardMode,
            Mode::Scientific => Command::IsScientificMode,
            Mode::Programmer => Command::IsProgrammerMode,
        })
    }
    /// Validate the complete input before changing the native state. Never parse through f64.
    pub fn enter(&mut self, text: &str) -> Result<()> {
        let mut commands = Vec::new();
        for ch in text.chars() {
            commands.push(match ch {
                '0' => Command::Zero,
                '1' => Command::One,
                '2' => Command::Two,
                '3' => Command::Three,
                '4' => Command::Four,
                '5' => Command::Five,
                '6' => Command::Six,
                '7' => Command::Seven,
                '8' => Command::Eight,
                '9' => Command::Nine,
                'A' | 'a' => Command::A,
                'B' | 'b' => Command::B,
                'C' | 'c' => Command::C,
                'D' | 'd' => Command::D,
                'E' | 'e' => Command::E,
                'F' | 'f' => Command::F,
                '.' => Command::Decimal,
                '+' => Command::Add,
                '-' | '−' => Command::Subtract,
                '*' | '×' => Command::Multiply,
                '/' | '÷' => Command::Divide,
                '=' => Command::Equals,
                '(' => Command::OpenParenthesis,
                ')' => Command::CloseParenthesis,
                ' ' | ',' | '\r' | '\n' | '\t' => continue,
                _ => return Err(EngineError(format!("Unsupported input character: {ch}"))),
            });
        }
        for command in commands {
            self.send(command)?;
        }
        Ok(())
    }
    pub fn store_memory(&mut self) -> Result<()> {
        self.action(1, 0, 0)
    }
    /// Import one clipboard number using native commands, preserving all decimal digits.
    /// Validate before clearing so invalid clipboard content cannot destroy an expression.
    pub fn paste_number(&mut self, text: &str) -> Result<()> {
        let invalid =
            || EngineError("Clipboard does not contain a number in the current base".into());
        let text = text.trim();
        if text.is_empty() || text.len() > 4096 {
            return Err(invalid());
        }
        let negative = text.starts_with(['-', '−']);
        let unsigned = text.strip_prefix(['-', '−', '+']).unwrap_or(text);
        let mut commands = Vec::new();
        if self.mode == Mode::Programmer {
            for ch in unsigned.chars().filter(|c| !c.is_ascii_whitespace()) {
                let digit = ch.to_digit(self.radix).ok_or_else(invalid)?;
                commands.push(
                    Command::from_name(
                        [
                            "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
                            "Nine", "A", "B", "C", "D", "E", "F",
                        ][digit as usize],
                    )
                    .expect("digit command"),
                );
            }
            if negative {
                commands.push(Command::Negate);
            }
        } else {
            let mut parts = unsigned.split(['e', 'E']);
            let mantissa = parts.next().ok_or_else(invalid)?;
            let exponent = parts.next();
            if parts.next().is_some() {
                return Err(invalid());
            }
            let mut dot = false;
            let mut digits = 0;
            let integer = mantissa.split('.').next().ok_or_else(invalid)?;
            if integer.contains(',') {
                let groups = integer.split(',').collect::<Vec<_>>();
                if groups[0].is_empty()
                    || groups[0].len() > 3
                    || groups[1..].iter().any(|g| g.len() != 3)
                {
                    return Err(invalid());
                }
            }
            for ch in mantissa.chars() {
                if ch == ',' && !dot {
                    continue;
                }
                if ch == '.' && !dot {
                    dot = true;
                    commands.push(Command::Decimal);
                    continue;
                }
                let digit = ch.to_digit(10).ok_or_else(invalid)?;
                digits += 1;
                commands.push(
                    Command::from_name(
                        [
                            "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
                            "Nine",
                        ][digit as usize],
                    )
                    .expect("digit command"),
                );
            }
            if digits == 0 {
                return Err(invalid());
            }
            if negative {
                commands.push(Command::Negate);
            }
            if let Some(exp) = exponent {
                let neg = exp.starts_with('-');
                let exp = exp.strip_prefix(['-', '+']).unwrap_or(exp);
                if exp.is_empty() || exp.len() > 4 {
                    return Err(invalid());
                }
                commands.push(Command::Exp);
                for ch in exp.chars() {
                    let digit = ch.to_digit(10).ok_or_else(invalid)?;
                    commands.push(
                        Command::from_name(
                            [
                                "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven",
                                "Eight", "Nine",
                            ][digit as usize],
                        )
                        .expect("digit command"),
                    );
                }
                if neg {
                    commands.push(Command::Negate);
                }
            }
        }
        if commands.is_empty() {
            return Err(invalid());
        }
        self.send(Command::ClearEntry)?;
        for command in commands {
            self.send(command)?;
        }
        Ok(())
    }
    pub fn recall_memory(&mut self, index: usize) -> Result<()> {
        self.index_action(2, index)
    }
    pub fn add_memory(&mut self, index: usize) -> Result<()> {
        self.index_action(3, index)
    }
    pub fn subtract_memory(&mut self, index: usize) -> Result<()> {
        self.index_action(4, index)
    }
    pub fn remove_memory(&mut self, index: usize) -> Result<()> {
        self.index_action(5, index)
    }
    pub fn clear_memory(&mut self) -> Result<()> {
        self.action(6, 0, 0)
    }
    pub fn clear_history(&mut self) -> Result<()> {
        self.action(7, 0, 0)
    }
    pub fn remove_history(&mut self, index: usize) -> Result<()> {
        self.index_action(8, index)
    }
    pub fn recall_history(&mut self, index: usize) -> Result<()> {
        self.index_action(9, index)
    }
    fn index_action(&mut self, action: i32, index: usize) -> Result<()> {
        let index =
            i32::try_from(index).map_err(|_| EngineError("Item index out of range".into()))?;
        self.action(action, index, 0)
    }
    pub fn set_category(&mut self, id: i32) -> Result<()> {
        self.action(10, id, 0)
    }
    pub fn set_units(&mut self, from: i32, to: i32) -> Result<()> {
        self.action(11, from, to)
    }
    pub fn converter_input(&mut self, text: &str) -> Result<()> {
        let commands = text
            .chars()
            .map(|c| match c {
                '0'..='9' => Ok(c as i32 - '0' as i32),
                '.' => Ok(10),
                '-' => Ok(11),
                _ => Err(EngineError(format!("Unsupported converter input: {c}"))),
            })
            .collect::<Result<Vec<_>>>()?;
        for command in commands {
            self.action(12, command, 0)?;
        }
        Ok(())
    }
    pub fn converter_clear(&mut self) -> Result<()> {
        self.action(12, 13, 0)
    }
    pub fn converter_backspace(&mut self) -> Result<()> {
        self.action(12, 12, 0)
    }
    pub fn converter_swap(&mut self) -> Result<()> {
        self.action(13, 0, 0)
    }
    pub fn toggle_bit(&mut self, bit: u8) -> Result<()> {
        if bit >= 64 {
            return Err(EngineError("Bit index must be below 64".into()));
        }
        self.action(0, 700 + i32::from(bit), 0)
    }
}
impl Drop for Engine {
    fn drop(&mut self) {
        let _guard = ENGINE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: the Context is owned exactly once and no other references escape this wrapper.
        unsafe {
            ferro_calc_destroy(self.handle.as_ptr());
        }
    }
}
