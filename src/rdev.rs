#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::SystemTime;
use std::{fmt, fmt::Display};

#[cfg(feature = "typescript")]
use specta::Type;

#[cfg(feature = "python")]
use pyo3::prelude::*;

// /// Callback type to send to listen function.
// pub type Callback = dyn FnMut(Event) -> ();

/// Callback type to send to grab function.
pub type GrabCallback = fn(event: Event) -> Option<Event>;

/// Errors that occur when trying to capture OS events.
/// Be careful on Mac, not setting accessibility does not cause an error
/// it justs ignores events.
#[derive(Debug)]
#[non_exhaustive]
pub enum ListenError {
    /// MacOS
    EventTapError,
    /// MacOS
    LoopSourceError,
    /// Linux
    MissingDisplayError,
    /// Linux
    KeyboardError,
    /// Linux
    RecordContextEnablingError,
    /// Linux
    RecordContextError,
    /// Linux
    XRecordExtensionError,
    /// Windows
    KeyHookError(u32),
    /// Windows
    MouseHookError(u32),
}

/// Errors that occur when trying to grab OS events.
/// Be careful on Mac, not setting accessibility does not cause an error
/// it justs ignores events.
#[derive(Debug)]
#[non_exhaustive]
pub enum GrabError {
    /// MacOS
    EventTapError,
    /// MacOS
    LoopSourceError,
    /// Linux
    MissingDisplayError,
    /// Linux
    KeyboardError,
    /// Windows
    KeyHookError(u32),
    /// Windows
    MouseHookError(u32),
    /// All
    SimulateError,
    IoError(std::io::Error),
}
/// Errors that occur when trying to get display size.
#[non_exhaustive]
#[derive(Debug)]
pub enum DisplayError {
    NoDisplay,
    ConversionError,
}

impl From<SimulateError> for GrabError {
    fn from(_: SimulateError) -> GrabError {
        GrabError::SimulateError
    }
}

impl From<std::io::Error> for GrabError {
    fn from(err: std::io::Error) -> GrabError {
        GrabError::IoError(err)
    }
}

/// Marking an error when we tried to simulate and event
#[derive(Debug)]
pub struct SimulateError;

impl Display for SimulateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not simulate event")
    }
}

impl std::error::Error for SimulateError {}

/// Key names based on physical location on the device
/// Merge Option(MacOS) and Alt(Windows, Linux) into Alt
/// Merge Windows (Windows), Meta(Linux), Command(MacOS) into Meta
/// Characters based on Qwerty layout, don't use this for characters as it WILL
/// depend on the layout. Use Event.name instead. Key modifiers gives those keys
/// a different value too.
/// Careful, on Windows KpReturn does not exist, it' s strictly equivalent to Return, also Keypad keys
/// get modified if NumLock is Off and ARE pagedown and so on.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "typescript", derive(Type))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Key {
    /// Alt key on Linux and Windows (option key on macOS)
    Alt,
    AltGr,
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    DownArrow,
    End,
    Escape,
    F1,
    F10,
    F11,
    F12,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    Home,
    LeftArrow,
    /// also known as "windows", "super", and "command"
    MetaLeft,
    /// also known as "windows", "super", and "command"
    MetaRight,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    UpArrow,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    LeftBracket,
    RightBracket,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Dot,
    Slash,
    Insert,
    KpReturn,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDelete,
    Function,
    Unknown(u32),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseKeyError;

impl FromStr for Key {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Alt" => Key::Alt,
            "AltGr" => Key::AltGr,
            "Backspace" => Key::Backspace,
            "CapsLock" => Key::CapsLock,
            "ControlLeft" => Key::ControlLeft,
            "ControlRight" => Key::ControlRight,
            "Delete" => Key::Delete,
            "DownArrow" => Key::DownArrow,
            "End" => Key::End,
            "Escape" => Key::Escape,
            "F1" => Key::F1,
            "F10" => Key::F10,
            "F11" => Key::F11,
            "F12" => Key::F12,
            "F2" => Key::F2,
            "F3" => Key::F3,
            "F4" => Key::F4,
            "F5" => Key::F5,
            "F6" => Key::F6,
            "F7" => Key::F7,
            "F8" => Key::F8,
            "F9" => Key::F9,
            "Home" => Key::Home,
            "LeftArrow" => Key::LeftArrow,
            "MetaLeft" => Key::MetaLeft,
            "MetaRight" => Key::MetaRight,
            "PageDown" => Key::PageDown,
            "PageUp" => Key::PageUp,
            "Return" => Key::Return,
            "RightArrow" => Key::RightArrow,
            "ShiftLeft" => Key::ShiftLeft,
            "ShiftRight" => Key::ShiftRight,
            "Space" => Key::Space,
            "Tab" => Key::Tab,
            "UpArrow" => Key::UpArrow,
            "PrintScreen" => Key::PrintScreen,
            "ScrollLock" => Key::ScrollLock,
            "Pause" => Key::Pause,
            "NumLock" => Key::NumLock,
            "BackQuote" => Key::BackQuote,
            "Num1" => Key::Num1,
            "Num2" => Key::Num2,
            "Num3" => Key::Num3,
            "Num4" => Key::Num4,
            "Num5" => Key::Num5,
            "Num6" => Key::Num6,
            "Num7" => Key::Num7,
            "Num8" => Key::Num8,
            "Num9" => Key::Num9,
            "Num0" => Key::Num0,
            "Minus" => Key::Minus,
            "Equal" => Key::Equal,
            "KeyQ" => Key::KeyQ,
            "KeyW" => Key::KeyW,
            "KeyE" => Key::KeyE,
            "KeyR" => Key::KeyR,
            "KeyT" => Key::KeyT,
            "KeyY" => Key::KeyY,
            "KeyU" => Key::KeyU,
            "KeyI" => Key::KeyI,
            "KeyO" => Key::KeyO,
            "KeyP" => Key::KeyP,
            "LeftBracket" => Key::LeftBracket,
            "RightBracket" => Key::RightBracket,
            "KeyA" => Key::KeyA,
            "KeyS" => Key::KeyS,
            "KeyD" => Key::KeyD,
            "KeyF" => Key::KeyF,
            "KeyG" => Key::KeyG,
            "KeyH" => Key::KeyH,
            "KeyJ" => Key::KeyJ,
            "KeyK" => Key::KeyK,
            "KeyL" => Key::KeyL,
            "SemiColon" => Key::SemiColon,
            "Quote" => Key::Quote,
            "BackSlash" => Key::BackSlash,
            "IntlBackslash" => Key::IntlBackslash,
            "KeyZ" => Key::KeyZ,
            "KeyX" => Key::KeyX,
            "KeyC" => Key::KeyC,
            "KeyV" => Key::KeyV,
            "KeyB" => Key::KeyB,
            "KeyN" => Key::KeyN,
            "KeyM" => Key::KeyM,
            "Comma" => Key::Comma,
            "Dot" => Key::Dot,
            "Slash" => Key::Slash,
            "Insert" => Key::Insert,
            "KpReturn" => Key::KpReturn,
            "KpMinus" => Key::KpMinus,
            "KpPlus" => Key::KpPlus,
            "KpMultiply" => Key::KpMultiply,
            "KpDivide" => Key::KpDivide,
            "Kp0" => Key::Kp0,
            "Kp1" => Key::Kp1,
            "Kp2" => Key::Kp2,
            "Kp3" => Key::Kp3,
            "Kp4" => Key::Kp4,
            "Kp5" => Key::Kp5,
            "Kp6" => Key::Kp6,
            "Kp7" => Key::Kp7,
            "Kp8" => Key::Kp8,
            "Kp9" => Key::Kp9,
            "KpDelete" => Key::KpDelete,
            "Function" => Key::Function,
            unknown => {
                if let Some(id) = unknown.strip_prefix("Unknown(") {
                    if let Some(id) = id.strip_suffix(')') {
                        if let Ok(id) = id.parse() {
                            return Ok(Key::Unknown(id));
                        }
                    }
                }

                return Err(ParseKeyError);
            }
        })
    }
}

impl ToString for Key {
    fn to_string(&self) -> String {
        match self {
            Key::Alt => "Alt".into(),
            Key::AltGr => "AltGr".into(),
            Key::Backspace => "Backspace".into(),
            Key::CapsLock => "CapsLock".into(),
            Key::ControlLeft => "ControlLeft".into(),
            Key::ControlRight => "ControlRight".into(),
            Key::Delete => "Delete".into(),
            Key::DownArrow => "DownArrow".into(),
            Key::End => "End".into(),
            Key::Escape => "Escape".into(),
            Key::F1 => "F1".into(),
            Key::F10 => "F10".into(),
            Key::F11 => "F11".into(),
            Key::F12 => "F12".into(),
            Key::F2 => "F2".into(),
            Key::F3 => "F3".into(),
            Key::F4 => "F4".into(),
            Key::F5 => "F5".into(),
            Key::F6 => "F6".into(),
            Key::F7 => "F7".into(),
            Key::F8 => "F8".into(),
            Key::F9 => "F9".into(),
            Key::Home => "Home".into(),
            Key::LeftArrow => "LeftArrow".into(),
            Key::MetaLeft => "MetaLeft".into(),
            Key::MetaRight => "MetaRight".into(),
            Key::PageDown => "PageDown".into(),
            Key::PageUp => "PageUp".into(),
            Key::Return => "Return".into(),
            Key::RightArrow => "RightArrow".into(),
            Key::ShiftLeft => "ShiftLeft".into(),
            Key::ShiftRight => "ShiftRight".into(),
            Key::Space => "Space".into(),
            Key::Tab => "Tab".into(),
            Key::UpArrow => "UpArrow".into(),
            Key::PrintScreen => "PrintScreen".into(),
            Key::ScrollLock => "ScrollLock".into(),
            Key::Pause => "Pause".into(),
            Key::NumLock => "NumLock".into(),
            Key::BackQuote => "BackQuote".into(),
            Key::Num1 => "Num1".into(),
            Key::Num2 => "Num2".into(),
            Key::Num3 => "Num3".into(),
            Key::Num4 => "Num4".into(),
            Key::Num5 => "Num5".into(),
            Key::Num6 => "Num6".into(),
            Key::Num7 => "Num7".into(),
            Key::Num8 => "Num8".into(),
            Key::Num9 => "Num9".into(),
            Key::Num0 => "Num0".into(),
            Key::Minus => "Minus".into(),
            Key::Equal => "Equal".into(),
            Key::KeyQ => "KeyQ".into(),
            Key::KeyW => "KeyW".into(),
            Key::KeyE => "KeyE".into(),
            Key::KeyR => "KeyR".into(),
            Key::KeyT => "KeyT".into(),
            Key::KeyY => "KeyY".into(),
            Key::KeyU => "KeyU".into(),
            Key::KeyI => "KeyI".into(),
            Key::KeyO => "KeyO".into(),
            Key::KeyP => "KeyP".into(),
            Key::LeftBracket => "LeftBracket".into(),
            Key::RightBracket => "RightBracket".into(),
            Key::KeyA => "KeyA".into(),
            Key::KeyS => "KeyS".into(),
            Key::KeyD => "KeyD".into(),
            Key::KeyF => "KeyF".into(),
            Key::KeyG => "KeyG".into(),
            Key::KeyH => "KeyH".into(),
            Key::KeyJ => "KeyJ".into(),
            Key::KeyK => "KeyK".into(),
            Key::KeyL => "KeyL".into(),
            Key::SemiColon => "SemiColon".into(),
            Key::Quote => "Quote".into(),
            Key::BackSlash => "BackSlash".into(),
            Key::IntlBackslash => "IntlBackslash".into(),
            Key::KeyZ => "KeyZ".into(),
            Key::KeyX => "KeyX".into(),
            Key::KeyC => "KeyC".into(),
            Key::KeyV => "KeyV".into(),
            Key::KeyB => "KeyB".into(),
            Key::KeyN => "KeyN".into(),
            Key::KeyM => "KeyM".into(),
            Key::Comma => "Comma".into(),
            Key::Dot => "Dot".into(),
            Key::Slash => "Slash".into(),
            Key::Insert => "Insert".into(),
            Key::KpReturn => "KpReturn".into(),
            Key::KpMinus => "KpMinus".into(),
            Key::KpPlus => "KpPlus".into(),
            Key::KpMultiply => "KpMultiply".into(),
            Key::KpDivide => "KpDivide".into(),
            Key::Kp0 => "Kp0".into(),
            Key::Kp1 => "Kp1".into(),
            Key::Kp2 => "Kp2".into(),
            Key::Kp3 => "Kp3".into(),
            Key::Kp4 => "Kp4".into(),
            Key::Kp5 => "Kp5".into(),
            Key::Kp6 => "Kp6".into(),
            Key::Kp7 => "Kp7".into(),
            Key::Kp8 => "Kp8".into(),
            Key::Kp9 => "Kp9".into(),
            Key::KpDelete => "KpDelete".into(),
            Key::Function => "Function".into(),
            Key::Unknown(id) => format!("Unknown({})", id),
        }
    }
}

/// Standard mouse buttons
/// Some mice have more than 3 buttons. These are not defined, and different
/// OSs will give different `Button::Unknown` values.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "typescript", derive(Type))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Button {
    Left,
    Right,
    Middle,
    Unknown(u8),
}

/// In order to manage different OSs, the current EventType choices are a mix and
/// match to account for all possible events.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "typescript", derive(Type))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum EventType {
    /// The keys correspond to a standard qwerty layout, they don't correspond
    /// To the actual letter a user would use, that requires some layout logic to be added.
    KeyPress(Key),
    KeyRelease(Key),
    /// Mouse Button
    ButtonPress(Button),
    ButtonRelease(Button),
    /// Values in pixels. `EventType::MouseMove{x: 0, y: 0}` corresponds to the
    /// top left corner, with x increasing downward and y increasing rightward
    MouseMove {
        x: f64,
        y: f64,
    },
    /// `delta_y` represents vertical scroll and `delta_x` represents horizontal scroll.
    /// Positive values correspond to scrolling up or right and negative values
    /// correspond to scrolling down or left
    Wheel {
        delta_x: i64,
        delta_y: i64,
    },
}

/// When events arrive from the OS they get some additional information added from
/// EventType, which is the time when this event was received, and the name Option
/// which contains what characters should be emmitted from that event. This relies
/// on the OS layout and keyboard state machinery.
/// Caveat: Dead keys don't function on Linux(X11) yet. You will receive None for
/// a dead key, and the raw letter instead of accentuated letter.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "typescript", derive(Type))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Event {
    pub time: SystemTime,
    pub name: Option<String>,
    pub event_type: EventType,
}

/// We can define a dummy Keyboard, that we will use to detect
/// what kind of EventType trigger some String. We get the currently used
/// layout for now !
/// Caveat : This is layout dependent. If your app needs to support
/// layout switching don't use this !
/// Caveat: On Linux, the dead keys mechanism is not implemented.
/// Caveat: Only shift and dead keys are implemented, Alt+unicode code on windows
/// won't work.
///
/// ```no_run
/// use rdev::{Keyboard, EventType, Key, KeyboardState};
///
/// let mut keyboard = Keyboard::new().unwrap();
/// let string = keyboard.add(&EventType::KeyPress(Key::KeyS));
/// // string == Some("s")
/// ```
pub trait KeyboardState {
    /// Changes the keyboard state as if this event happened. we don't
    /// really hit the OS here, which might come handy to test what should happen
    /// if we were to hit said key.
    fn add(&mut self, event_type: &EventType) -> Option<String>;

    /// Resets the keyboard state as if we never touched it (no shift, caps_lock and so on)
    fn reset(&mut self);
}
