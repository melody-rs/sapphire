// this is so fucking scuffed lmao
#[derive(
    num_enum::IntoPrimitive,
    num_enum::TryFromPrimitive,
    strum::EnumIter,
    strum::IntoStaticStr,
    Clone,
    Copy,
)]
#[repr(u16)]
pub enum KeyCode {
    Backquote,
    Backslash,
    BracketLeft,
    BracketRight,
    Comma,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Equal,
    IntlBackslash,
    IntlRo,
    IntlYen,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,
    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ContextMenu,
    ControlLeft,
    ControlRight,
    Enter,
    SuperLeft,
    SuperRight,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    Convert,
    KanaMode,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    NonConvert,
    Delete,
    End,
    Help,
    Home,
    Insert,
    PageDown,
    PageUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadBackspace,
    NumpadClear,
    NumpadClearEntry,
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadHash,
    NumpadMemoryAdd,
    NumpadMemoryClear,
    NumpadMemoryRecall,
    NumpadMemoryStore,
    NumpadMemorySubtract,
    NumpadMultiply,
    NumpadParenLeft,
    NumpadParenRight,
    NumpadStar,
    NumpadSubtract,
    Escape,
    Fn,
    FnLock,
    PrintScreen,
    ScrollLock,
    Pause,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    Eject,
    LaunchApp1,
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    Meta,
    Hyper,
    Turbo,
    Abort,
    Resume,
    Suspend,
    Again,
    Copy,
    Cut,
    Find,
    Open,
    Paste,
    Props,
    Select,
    Undo,
    Hiragana,
    Katakana,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
}

impl From<winit::keyboard::KeyCode> for KeyCode {
    fn from(value: winit::keyboard::KeyCode) -> Self {
        match value {
            winit::keyboard::KeyCode::Backquote => KeyCode::Backquote,
            winit::keyboard::KeyCode::Backslash => KeyCode::Backslash,
            winit::keyboard::KeyCode::BracketLeft => KeyCode::BracketLeft,
            winit::keyboard::KeyCode::BracketRight => KeyCode::BracketRight,
            winit::keyboard::KeyCode::Comma => KeyCode::Comma,
            winit::keyboard::KeyCode::Digit0 => KeyCode::Digit0,
            winit::keyboard::KeyCode::Digit1 => KeyCode::Digit1,
            winit::keyboard::KeyCode::Digit2 => KeyCode::Digit2,
            winit::keyboard::KeyCode::Digit3 => KeyCode::Digit3,
            winit::keyboard::KeyCode::Digit4 => KeyCode::Digit4,
            winit::keyboard::KeyCode::Digit5 => KeyCode::Digit5,
            winit::keyboard::KeyCode::Digit6 => KeyCode::Digit6,
            winit::keyboard::KeyCode::Digit7 => KeyCode::Digit7,
            winit::keyboard::KeyCode::Digit8 => KeyCode::Digit8,
            winit::keyboard::KeyCode::Digit9 => KeyCode::Digit9,
            winit::keyboard::KeyCode::Equal => KeyCode::Equal,
            winit::keyboard::KeyCode::IntlBackslash => KeyCode::IntlBackslash,
            winit::keyboard::KeyCode::IntlRo => KeyCode::IntlRo,
            winit::keyboard::KeyCode::IntlYen => KeyCode::IntlYen,
            winit::keyboard::KeyCode::KeyA => KeyCode::KeyA,
            winit::keyboard::KeyCode::KeyB => KeyCode::KeyB,
            winit::keyboard::KeyCode::KeyC => KeyCode::KeyC,
            winit::keyboard::KeyCode::KeyD => KeyCode::KeyD,
            winit::keyboard::KeyCode::KeyE => KeyCode::KeyE,
            winit::keyboard::KeyCode::KeyF => KeyCode::KeyF,
            winit::keyboard::KeyCode::KeyG => KeyCode::KeyG,
            winit::keyboard::KeyCode::KeyH => KeyCode::KeyH,
            winit::keyboard::KeyCode::KeyI => KeyCode::KeyI,
            winit::keyboard::KeyCode::KeyJ => KeyCode::KeyJ,
            winit::keyboard::KeyCode::KeyK => KeyCode::KeyK,
            winit::keyboard::KeyCode::KeyL => KeyCode::KeyL,
            winit::keyboard::KeyCode::KeyM => KeyCode::KeyM,
            winit::keyboard::KeyCode::KeyN => KeyCode::KeyN,
            winit::keyboard::KeyCode::KeyO => KeyCode::KeyO,
            winit::keyboard::KeyCode::KeyP => KeyCode::KeyP,
            winit::keyboard::KeyCode::KeyQ => KeyCode::KeyQ,
            winit::keyboard::KeyCode::KeyR => KeyCode::KeyR,
            winit::keyboard::KeyCode::KeyS => KeyCode::KeyS,
            winit::keyboard::KeyCode::KeyT => KeyCode::KeyT,
            winit::keyboard::KeyCode::KeyU => KeyCode::KeyU,
            winit::keyboard::KeyCode::KeyV => KeyCode::KeyV,
            winit::keyboard::KeyCode::KeyW => KeyCode::KeyW,
            winit::keyboard::KeyCode::KeyX => KeyCode::KeyX,
            winit::keyboard::KeyCode::KeyY => KeyCode::KeyY,
            winit::keyboard::KeyCode::KeyZ => KeyCode::KeyZ,
            winit::keyboard::KeyCode::Minus => KeyCode::Minus,
            winit::keyboard::KeyCode::Period => KeyCode::Period,
            winit::keyboard::KeyCode::Quote => KeyCode::Quote,
            winit::keyboard::KeyCode::Semicolon => KeyCode::Semicolon,
            winit::keyboard::KeyCode::Slash => KeyCode::Slash,
            winit::keyboard::KeyCode::AltLeft => KeyCode::AltLeft,
            winit::keyboard::KeyCode::AltRight => KeyCode::AltRight,
            winit::keyboard::KeyCode::Backspace => KeyCode::Backspace,
            winit::keyboard::KeyCode::CapsLock => KeyCode::CapsLock,
            winit::keyboard::KeyCode::ContextMenu => KeyCode::ContextMenu,
            winit::keyboard::KeyCode::ControlLeft => KeyCode::ControlLeft,
            winit::keyboard::KeyCode::ControlRight => KeyCode::ControlRight,
            winit::keyboard::KeyCode::Enter => KeyCode::Enter,
            winit::keyboard::KeyCode::SuperLeft => KeyCode::SuperLeft,
            winit::keyboard::KeyCode::SuperRight => KeyCode::SuperRight,
            winit::keyboard::KeyCode::ShiftLeft => KeyCode::ShiftLeft,
            winit::keyboard::KeyCode::ShiftRight => KeyCode::ShiftRight,
            winit::keyboard::KeyCode::Space => KeyCode::Space,
            winit::keyboard::KeyCode::Tab => KeyCode::Tab,
            winit::keyboard::KeyCode::Convert => KeyCode::Convert,
            winit::keyboard::KeyCode::KanaMode => KeyCode::KanaMode,
            winit::keyboard::KeyCode::Lang1 => KeyCode::Lang1,
            winit::keyboard::KeyCode::Lang2 => KeyCode::Lang2,
            winit::keyboard::KeyCode::Lang3 => KeyCode::Lang3,
            winit::keyboard::KeyCode::Lang4 => KeyCode::Lang4,
            winit::keyboard::KeyCode::Lang5 => KeyCode::Lang5,
            winit::keyboard::KeyCode::NonConvert => KeyCode::NonConvert,
            winit::keyboard::KeyCode::Delete => KeyCode::Delete,
            winit::keyboard::KeyCode::End => KeyCode::End,
            winit::keyboard::KeyCode::Help => KeyCode::Help,
            winit::keyboard::KeyCode::Home => KeyCode::Home,
            winit::keyboard::KeyCode::Insert => KeyCode::Insert,
            winit::keyboard::KeyCode::PageDown => KeyCode::PageDown,
            winit::keyboard::KeyCode::PageUp => KeyCode::PageUp,
            winit::keyboard::KeyCode::ArrowDown => KeyCode::ArrowDown,
            winit::keyboard::KeyCode::ArrowLeft => KeyCode::ArrowLeft,
            winit::keyboard::KeyCode::ArrowRight => KeyCode::ArrowRight,
            winit::keyboard::KeyCode::ArrowUp => KeyCode::ArrowUp,
            winit::keyboard::KeyCode::NumLock => KeyCode::NumLock,
            winit::keyboard::KeyCode::Numpad0 => KeyCode::Numpad0,
            winit::keyboard::KeyCode::Numpad1 => KeyCode::Numpad1,
            winit::keyboard::KeyCode::Numpad2 => KeyCode::Numpad2,
            winit::keyboard::KeyCode::Numpad3 => KeyCode::Numpad3,
            winit::keyboard::KeyCode::Numpad4 => KeyCode::Numpad4,
            winit::keyboard::KeyCode::Numpad5 => KeyCode::Numpad5,
            winit::keyboard::KeyCode::Numpad6 => KeyCode::Numpad6,
            winit::keyboard::KeyCode::Numpad7 => KeyCode::Numpad7,
            winit::keyboard::KeyCode::Numpad8 => KeyCode::Numpad8,
            winit::keyboard::KeyCode::Numpad9 => KeyCode::Numpad9,
            winit::keyboard::KeyCode::NumpadAdd => KeyCode::NumpadAdd,
            winit::keyboard::KeyCode::NumpadBackspace => KeyCode::NumpadBackspace,
            winit::keyboard::KeyCode::NumpadClear => KeyCode::NumpadClear,
            winit::keyboard::KeyCode::NumpadClearEntry => KeyCode::NumpadClearEntry,
            winit::keyboard::KeyCode::NumpadComma => KeyCode::NumpadComma,
            winit::keyboard::KeyCode::NumpadDecimal => KeyCode::NumpadDecimal,
            winit::keyboard::KeyCode::NumpadDivide => KeyCode::NumpadDivide,
            winit::keyboard::KeyCode::NumpadEnter => KeyCode::NumpadEnter,
            winit::keyboard::KeyCode::NumpadEqual => KeyCode::NumpadEqual,
            winit::keyboard::KeyCode::NumpadHash => KeyCode::NumpadHash,
            winit::keyboard::KeyCode::NumpadMemoryAdd => KeyCode::NumpadMemoryAdd,
            winit::keyboard::KeyCode::NumpadMemoryClear => KeyCode::NumpadMemoryClear,
            winit::keyboard::KeyCode::NumpadMemoryRecall => KeyCode::NumpadMemoryRecall,
            winit::keyboard::KeyCode::NumpadMemoryStore => KeyCode::NumpadMemoryStore,
            winit::keyboard::KeyCode::NumpadMemorySubtract => KeyCode::NumpadMemorySubtract,
            winit::keyboard::KeyCode::NumpadMultiply => KeyCode::NumpadMultiply,
            winit::keyboard::KeyCode::NumpadParenLeft => KeyCode::NumpadParenLeft,
            winit::keyboard::KeyCode::NumpadParenRight => KeyCode::NumpadParenRight,
            winit::keyboard::KeyCode::NumpadStar => KeyCode::NumpadStar,
            winit::keyboard::KeyCode::NumpadSubtract => KeyCode::NumpadSubtract,
            winit::keyboard::KeyCode::Escape => KeyCode::Escape,
            winit::keyboard::KeyCode::Fn => KeyCode::Fn,
            winit::keyboard::KeyCode::FnLock => KeyCode::FnLock,
            winit::keyboard::KeyCode::PrintScreen => KeyCode::PrintScreen,
            winit::keyboard::KeyCode::ScrollLock => KeyCode::ScrollLock,
            winit::keyboard::KeyCode::Pause => KeyCode::Pause,
            winit::keyboard::KeyCode::BrowserBack => KeyCode::BrowserBack,
            winit::keyboard::KeyCode::BrowserFavorites => KeyCode::BrowserFavorites,
            winit::keyboard::KeyCode::BrowserForward => KeyCode::BrowserForward,
            winit::keyboard::KeyCode::BrowserHome => KeyCode::BrowserHome,
            winit::keyboard::KeyCode::BrowserRefresh => KeyCode::BrowserRefresh,
            winit::keyboard::KeyCode::BrowserSearch => KeyCode::BrowserSearch,
            winit::keyboard::KeyCode::BrowserStop => KeyCode::BrowserStop,
            winit::keyboard::KeyCode::Eject => KeyCode::Eject,
            winit::keyboard::KeyCode::LaunchApp1 => KeyCode::LaunchApp1,
            winit::keyboard::KeyCode::LaunchApp2 => KeyCode::LaunchApp2,
            winit::keyboard::KeyCode::LaunchMail => KeyCode::LaunchMail,
            winit::keyboard::KeyCode::MediaPlayPause => KeyCode::MediaPlayPause,
            winit::keyboard::KeyCode::MediaSelect => KeyCode::MediaSelect,
            winit::keyboard::KeyCode::MediaStop => KeyCode::MediaStop,
            winit::keyboard::KeyCode::MediaTrackNext => KeyCode::MediaTrackNext,
            winit::keyboard::KeyCode::MediaTrackPrevious => KeyCode::MediaTrackPrevious,
            winit::keyboard::KeyCode::Power => KeyCode::Power,
            winit::keyboard::KeyCode::Sleep => KeyCode::Sleep,
            winit::keyboard::KeyCode::AudioVolumeDown => KeyCode::AudioVolumeDown,
            winit::keyboard::KeyCode::AudioVolumeMute => KeyCode::AudioVolumeMute,
            winit::keyboard::KeyCode::AudioVolumeUp => KeyCode::AudioVolumeUp,
            winit::keyboard::KeyCode::WakeUp => KeyCode::WakeUp,
            winit::keyboard::KeyCode::Meta => KeyCode::Meta,
            winit::keyboard::KeyCode::Hyper => KeyCode::Hyper,
            winit::keyboard::KeyCode::Turbo => KeyCode::Turbo,
            winit::keyboard::KeyCode::Abort => KeyCode::Abort,
            winit::keyboard::KeyCode::Resume => KeyCode::Resume,
            winit::keyboard::KeyCode::Suspend => KeyCode::Suspend,
            winit::keyboard::KeyCode::Again => KeyCode::Again,
            winit::keyboard::KeyCode::Copy => KeyCode::Copy,
            winit::keyboard::KeyCode::Cut => KeyCode::Cut,
            winit::keyboard::KeyCode::Find => KeyCode::Find,
            winit::keyboard::KeyCode::Open => KeyCode::Open,
            winit::keyboard::KeyCode::Paste => KeyCode::Paste,
            winit::keyboard::KeyCode::Props => KeyCode::Props,
            winit::keyboard::KeyCode::Select => KeyCode::Select,
            winit::keyboard::KeyCode::Undo => KeyCode::Undo,
            winit::keyboard::KeyCode::Hiragana => KeyCode::Hiragana,
            winit::keyboard::KeyCode::Katakana => KeyCode::Katakana,
            winit::keyboard::KeyCode::F1 => KeyCode::F1,
            winit::keyboard::KeyCode::F2 => KeyCode::F2,
            winit::keyboard::KeyCode::F3 => KeyCode::F3,
            winit::keyboard::KeyCode::F4 => KeyCode::F4,
            winit::keyboard::KeyCode::F5 => KeyCode::F5,
            winit::keyboard::KeyCode::F6 => KeyCode::F6,
            winit::keyboard::KeyCode::F7 => KeyCode::F7,
            winit::keyboard::KeyCode::F8 => KeyCode::F8,
            winit::keyboard::KeyCode::F9 => KeyCode::F9,
            winit::keyboard::KeyCode::F10 => KeyCode::F10,
            winit::keyboard::KeyCode::F11 => KeyCode::F11,
            winit::keyboard::KeyCode::F12 => KeyCode::F12,
            winit::keyboard::KeyCode::F13 => KeyCode::F13,
            winit::keyboard::KeyCode::F14 => KeyCode::F14,
            winit::keyboard::KeyCode::F15 => KeyCode::F15,
            winit::keyboard::KeyCode::F16 => KeyCode::F16,
            winit::keyboard::KeyCode::F17 => KeyCode::F17,
            winit::keyboard::KeyCode::F18 => KeyCode::F18,
            winit::keyboard::KeyCode::F19 => KeyCode::F19,
            winit::keyboard::KeyCode::F20 => KeyCode::F20,
            winit::keyboard::KeyCode::F21 => KeyCode::F21,
            winit::keyboard::KeyCode::F22 => KeyCode::F22,
            winit::keyboard::KeyCode::F23 => KeyCode::F23,
            winit::keyboard::KeyCode::F24 => KeyCode::F24,
            winit::keyboard::KeyCode::F25 => KeyCode::F25,
            winit::keyboard::KeyCode::F26 => KeyCode::F26,
            winit::keyboard::KeyCode::F27 => KeyCode::F27,
            winit::keyboard::KeyCode::F28 => KeyCode::F28,
            winit::keyboard::KeyCode::F29 => KeyCode::F29,
            winit::keyboard::KeyCode::F30 => KeyCode::F30,
            winit::keyboard::KeyCode::F31 => KeyCode::F31,
            winit::keyboard::KeyCode::F32 => KeyCode::F32,
            winit::keyboard::KeyCode::F33 => KeyCode::F33,
            winit::keyboard::KeyCode::F34 => KeyCode::F34,
            winit::keyboard::KeyCode::F35 => KeyCode::F35,
            _ => unimplemented!(),
        }
    }
}

impl From<KeyCode> for winit::keyboard::KeyCode {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Backquote => winit::keyboard::KeyCode::Backquote,
            KeyCode::Backslash => winit::keyboard::KeyCode::Backslash,
            KeyCode::BracketLeft => winit::keyboard::KeyCode::BracketLeft,
            KeyCode::BracketRight => winit::keyboard::KeyCode::BracketRight,
            KeyCode::Comma => winit::keyboard::KeyCode::Comma,
            KeyCode::Digit0 => winit::keyboard::KeyCode::Digit0,
            KeyCode::Digit1 => winit::keyboard::KeyCode::Digit1,
            KeyCode::Digit2 => winit::keyboard::KeyCode::Digit2,
            KeyCode::Digit3 => winit::keyboard::KeyCode::Digit3,
            KeyCode::Digit4 => winit::keyboard::KeyCode::Digit4,
            KeyCode::Digit5 => winit::keyboard::KeyCode::Digit5,
            KeyCode::Digit6 => winit::keyboard::KeyCode::Digit6,
            KeyCode::Digit7 => winit::keyboard::KeyCode::Digit7,
            KeyCode::Digit8 => winit::keyboard::KeyCode::Digit8,
            KeyCode::Digit9 => winit::keyboard::KeyCode::Digit9,
            KeyCode::Equal => winit::keyboard::KeyCode::Equal,
            KeyCode::IntlBackslash => winit::keyboard::KeyCode::IntlBackslash,
            KeyCode::IntlRo => winit::keyboard::KeyCode::IntlRo,
            KeyCode::IntlYen => winit::keyboard::KeyCode::IntlYen,
            KeyCode::KeyA => winit::keyboard::KeyCode::KeyA,
            KeyCode::KeyB => winit::keyboard::KeyCode::KeyB,
            KeyCode::KeyC => winit::keyboard::KeyCode::KeyC,
            KeyCode::KeyD => winit::keyboard::KeyCode::KeyD,
            KeyCode::KeyE => winit::keyboard::KeyCode::KeyE,
            KeyCode::KeyF => winit::keyboard::KeyCode::KeyF,
            KeyCode::KeyG => winit::keyboard::KeyCode::KeyG,
            KeyCode::KeyH => winit::keyboard::KeyCode::KeyH,
            KeyCode::KeyI => winit::keyboard::KeyCode::KeyI,
            KeyCode::KeyJ => winit::keyboard::KeyCode::KeyJ,
            KeyCode::KeyK => winit::keyboard::KeyCode::KeyK,
            KeyCode::KeyL => winit::keyboard::KeyCode::KeyL,
            KeyCode::KeyM => winit::keyboard::KeyCode::KeyM,
            KeyCode::KeyN => winit::keyboard::KeyCode::KeyN,
            KeyCode::KeyO => winit::keyboard::KeyCode::KeyO,
            KeyCode::KeyP => winit::keyboard::KeyCode::KeyP,
            KeyCode::KeyQ => winit::keyboard::KeyCode::KeyQ,
            KeyCode::KeyR => winit::keyboard::KeyCode::KeyR,
            KeyCode::KeyS => winit::keyboard::KeyCode::KeyS,
            KeyCode::KeyT => winit::keyboard::KeyCode::KeyT,
            KeyCode::KeyU => winit::keyboard::KeyCode::KeyU,
            KeyCode::KeyV => winit::keyboard::KeyCode::KeyV,
            KeyCode::KeyW => winit::keyboard::KeyCode::KeyW,
            KeyCode::KeyX => winit::keyboard::KeyCode::KeyX,
            KeyCode::KeyY => winit::keyboard::KeyCode::KeyY,
            KeyCode::KeyZ => winit::keyboard::KeyCode::KeyZ,
            KeyCode::Minus => winit::keyboard::KeyCode::Minus,
            KeyCode::Period => winit::keyboard::KeyCode::Period,
            KeyCode::Quote => winit::keyboard::KeyCode::Quote,
            KeyCode::Semicolon => winit::keyboard::KeyCode::Semicolon,
            KeyCode::Slash => winit::keyboard::KeyCode::Slash,
            KeyCode::AltLeft => winit::keyboard::KeyCode::AltLeft,
            KeyCode::AltRight => winit::keyboard::KeyCode::AltRight,
            KeyCode::Backspace => winit::keyboard::KeyCode::Backspace,
            KeyCode::CapsLock => winit::keyboard::KeyCode::CapsLock,
            KeyCode::ContextMenu => winit::keyboard::KeyCode::ContextMenu,
            KeyCode::ControlLeft => winit::keyboard::KeyCode::ControlLeft,
            KeyCode::ControlRight => winit::keyboard::KeyCode::ControlRight,
            KeyCode::Enter => winit::keyboard::KeyCode::Enter,
            KeyCode::SuperLeft => winit::keyboard::KeyCode::SuperLeft,
            KeyCode::SuperRight => winit::keyboard::KeyCode::SuperRight,
            KeyCode::ShiftLeft => winit::keyboard::KeyCode::ShiftLeft,
            KeyCode::ShiftRight => winit::keyboard::KeyCode::ShiftRight,
            KeyCode::Space => winit::keyboard::KeyCode::Space,
            KeyCode::Tab => winit::keyboard::KeyCode::Tab,
            KeyCode::Convert => winit::keyboard::KeyCode::Convert,
            KeyCode::KanaMode => winit::keyboard::KeyCode::KanaMode,
            KeyCode::Lang1 => winit::keyboard::KeyCode::Lang1,
            KeyCode::Lang2 => winit::keyboard::KeyCode::Lang2,
            KeyCode::Lang3 => winit::keyboard::KeyCode::Lang3,
            KeyCode::Lang4 => winit::keyboard::KeyCode::Lang4,
            KeyCode::Lang5 => winit::keyboard::KeyCode::Lang5,
            KeyCode::NonConvert => winit::keyboard::KeyCode::NonConvert,
            KeyCode::Delete => winit::keyboard::KeyCode::Delete,
            KeyCode::End => winit::keyboard::KeyCode::End,
            KeyCode::Help => winit::keyboard::KeyCode::Help,
            KeyCode::Home => winit::keyboard::KeyCode::Home,
            KeyCode::Insert => winit::keyboard::KeyCode::Insert,
            KeyCode::PageDown => winit::keyboard::KeyCode::PageDown,
            KeyCode::PageUp => winit::keyboard::KeyCode::PageUp,
            KeyCode::ArrowDown => winit::keyboard::KeyCode::ArrowDown,
            KeyCode::ArrowLeft => winit::keyboard::KeyCode::ArrowLeft,
            KeyCode::ArrowRight => winit::keyboard::KeyCode::ArrowRight,
            KeyCode::ArrowUp => winit::keyboard::KeyCode::ArrowUp,
            KeyCode::NumLock => winit::keyboard::KeyCode::NumLock,
            KeyCode::Numpad0 => winit::keyboard::KeyCode::Numpad0,
            KeyCode::Numpad1 => winit::keyboard::KeyCode::Numpad1,
            KeyCode::Numpad2 => winit::keyboard::KeyCode::Numpad2,
            KeyCode::Numpad3 => winit::keyboard::KeyCode::Numpad3,
            KeyCode::Numpad4 => winit::keyboard::KeyCode::Numpad4,
            KeyCode::Numpad5 => winit::keyboard::KeyCode::Numpad5,
            KeyCode::Numpad6 => winit::keyboard::KeyCode::Numpad6,
            KeyCode::Numpad7 => winit::keyboard::KeyCode::Numpad7,
            KeyCode::Numpad8 => winit::keyboard::KeyCode::Numpad8,
            KeyCode::Numpad9 => winit::keyboard::KeyCode::Numpad9,
            KeyCode::NumpadAdd => winit::keyboard::KeyCode::NumpadAdd,
            KeyCode::NumpadBackspace => winit::keyboard::KeyCode::NumpadBackspace,
            KeyCode::NumpadClear => winit::keyboard::KeyCode::NumpadClear,
            KeyCode::NumpadClearEntry => winit::keyboard::KeyCode::NumpadClearEntry,
            KeyCode::NumpadComma => winit::keyboard::KeyCode::NumpadComma,
            KeyCode::NumpadDecimal => winit::keyboard::KeyCode::NumpadDecimal,
            KeyCode::NumpadDivide => winit::keyboard::KeyCode::NumpadDivide,
            KeyCode::NumpadEnter => winit::keyboard::KeyCode::NumpadEnter,
            KeyCode::NumpadEqual => winit::keyboard::KeyCode::NumpadEqual,
            KeyCode::NumpadHash => winit::keyboard::KeyCode::NumpadHash,
            KeyCode::NumpadMemoryAdd => winit::keyboard::KeyCode::NumpadMemoryAdd,
            KeyCode::NumpadMemoryClear => winit::keyboard::KeyCode::NumpadMemoryClear,
            KeyCode::NumpadMemoryRecall => winit::keyboard::KeyCode::NumpadMemoryRecall,
            KeyCode::NumpadMemoryStore => winit::keyboard::KeyCode::NumpadMemoryStore,
            KeyCode::NumpadMemorySubtract => winit::keyboard::KeyCode::NumpadMemorySubtract,
            KeyCode::NumpadMultiply => winit::keyboard::KeyCode::NumpadMultiply,
            KeyCode::NumpadParenLeft => winit::keyboard::KeyCode::NumpadParenLeft,
            KeyCode::NumpadParenRight => winit::keyboard::KeyCode::NumpadParenRight,
            KeyCode::NumpadStar => winit::keyboard::KeyCode::NumpadStar,
            KeyCode::NumpadSubtract => winit::keyboard::KeyCode::NumpadSubtract,
            KeyCode::Escape => winit::keyboard::KeyCode::Escape,
            KeyCode::Fn => winit::keyboard::KeyCode::Fn,
            KeyCode::FnLock => winit::keyboard::KeyCode::FnLock,
            KeyCode::PrintScreen => winit::keyboard::KeyCode::PrintScreen,
            KeyCode::ScrollLock => winit::keyboard::KeyCode::ScrollLock,
            KeyCode::Pause => winit::keyboard::KeyCode::Pause,
            KeyCode::BrowserBack => winit::keyboard::KeyCode::BrowserBack,
            KeyCode::BrowserFavorites => winit::keyboard::KeyCode::BrowserFavorites,
            KeyCode::BrowserForward => winit::keyboard::KeyCode::BrowserForward,
            KeyCode::BrowserHome => winit::keyboard::KeyCode::BrowserHome,
            KeyCode::BrowserRefresh => winit::keyboard::KeyCode::BrowserRefresh,
            KeyCode::BrowserSearch => winit::keyboard::KeyCode::BrowserSearch,
            KeyCode::BrowserStop => winit::keyboard::KeyCode::BrowserStop,
            KeyCode::Eject => winit::keyboard::KeyCode::Eject,
            KeyCode::LaunchApp1 => winit::keyboard::KeyCode::LaunchApp1,
            KeyCode::LaunchApp2 => winit::keyboard::KeyCode::LaunchApp2,
            KeyCode::LaunchMail => winit::keyboard::KeyCode::LaunchMail,
            KeyCode::MediaPlayPause => winit::keyboard::KeyCode::MediaPlayPause,
            KeyCode::MediaSelect => winit::keyboard::KeyCode::MediaSelect,
            KeyCode::MediaStop => winit::keyboard::KeyCode::MediaStop,
            KeyCode::MediaTrackNext => winit::keyboard::KeyCode::MediaTrackNext,
            KeyCode::MediaTrackPrevious => winit::keyboard::KeyCode::MediaTrackPrevious,
            KeyCode::Power => winit::keyboard::KeyCode::Power,
            KeyCode::Sleep => winit::keyboard::KeyCode::Sleep,
            KeyCode::AudioVolumeDown => winit::keyboard::KeyCode::AudioVolumeDown,
            KeyCode::AudioVolumeMute => winit::keyboard::KeyCode::AudioVolumeMute,
            KeyCode::AudioVolumeUp => winit::keyboard::KeyCode::AudioVolumeUp,
            KeyCode::WakeUp => winit::keyboard::KeyCode::WakeUp,
            KeyCode::Meta => winit::keyboard::KeyCode::Meta,
            KeyCode::Hyper => winit::keyboard::KeyCode::Hyper,
            KeyCode::Turbo => winit::keyboard::KeyCode::Turbo,
            KeyCode::Abort => winit::keyboard::KeyCode::Abort,
            KeyCode::Resume => winit::keyboard::KeyCode::Resume,
            KeyCode::Suspend => winit::keyboard::KeyCode::Suspend,
            KeyCode::Again => winit::keyboard::KeyCode::Again,
            KeyCode::Copy => winit::keyboard::KeyCode::Copy,
            KeyCode::Cut => winit::keyboard::KeyCode::Cut,
            KeyCode::Find => winit::keyboard::KeyCode::Find,
            KeyCode::Open => winit::keyboard::KeyCode::Open,
            KeyCode::Paste => winit::keyboard::KeyCode::Paste,
            KeyCode::Props => winit::keyboard::KeyCode::Props,
            KeyCode::Select => winit::keyboard::KeyCode::Select,
            KeyCode::Undo => winit::keyboard::KeyCode::Undo,
            KeyCode::Hiragana => winit::keyboard::KeyCode::Hiragana,
            KeyCode::Katakana => winit::keyboard::KeyCode::Katakana,
            KeyCode::F1 => winit::keyboard::KeyCode::F1,
            KeyCode::F2 => winit::keyboard::KeyCode::F2,
            KeyCode::F3 => winit::keyboard::KeyCode::F3,
            KeyCode::F4 => winit::keyboard::KeyCode::F4,
            KeyCode::F5 => winit::keyboard::KeyCode::F5,
            KeyCode::F6 => winit::keyboard::KeyCode::F6,
            KeyCode::F7 => winit::keyboard::KeyCode::F7,
            KeyCode::F8 => winit::keyboard::KeyCode::F8,
            KeyCode::F9 => winit::keyboard::KeyCode::F9,
            KeyCode::F10 => winit::keyboard::KeyCode::F10,
            KeyCode::F11 => winit::keyboard::KeyCode::F11,
            KeyCode::F12 => winit::keyboard::KeyCode::F12,
            KeyCode::F13 => winit::keyboard::KeyCode::F13,
            KeyCode::F14 => winit::keyboard::KeyCode::F14,
            KeyCode::F15 => winit::keyboard::KeyCode::F15,
            KeyCode::F16 => winit::keyboard::KeyCode::F16,
            KeyCode::F17 => winit::keyboard::KeyCode::F17,
            KeyCode::F18 => winit::keyboard::KeyCode::F18,
            KeyCode::F19 => winit::keyboard::KeyCode::F19,
            KeyCode::F20 => winit::keyboard::KeyCode::F20,
            KeyCode::F21 => winit::keyboard::KeyCode::F21,
            KeyCode::F22 => winit::keyboard::KeyCode::F22,
            KeyCode::F23 => winit::keyboard::KeyCode::F23,
            KeyCode::F24 => winit::keyboard::KeyCode::F24,
            KeyCode::F25 => winit::keyboard::KeyCode::F25,
            KeyCode::F26 => winit::keyboard::KeyCode::F26,
            KeyCode::F27 => winit::keyboard::KeyCode::F27,
            KeyCode::F28 => winit::keyboard::KeyCode::F28,
            KeyCode::F29 => winit::keyboard::KeyCode::F29,
            KeyCode::F30 => winit::keyboard::KeyCode::F30,
            KeyCode::F31 => winit::keyboard::KeyCode::F31,
            KeyCode::F32 => winit::keyboard::KeyCode::F32,
            KeyCode::F33 => winit::keyboard::KeyCode::F33,
            KeyCode::F34 => winit::keyboard::KeyCode::F34,
            KeyCode::F35 => winit::keyboard::KeyCode::F35,
        }
    }
}
