
use winit::VirtualKeyCode;

pub enum GsKeycode {
    Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Escape,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15,
    Snapshot, Scroll, Pause, Insert, Home, Delete, End, PageDown, PageUp,
    Left, Up, Right, Down,
    Back, Return, Space, Compose, Caret, Tab,
    Numlock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    AbntC1, AbntC2, Add, Apostrophe, Apps, At, Ax, Backslash, Calculator, Capital, Colon, Comma, Convert, Decimal, Divide, Equals, Grave, Kana, Kanji,
    LAlt, LBracket, LControl, LShift, LWin,
    RAlt, RBracket, RControl, RShift, RWin,
    Mail, MediaSelect, MediaStop, Minus, Multiply, Mute, MyComputer, NavigateForward, NavigateBackward, NextTrack, NoConvert, NumpadComma, NumpadEnter, NumpadEquals, OEM102, Period, PlayPause, Power, PrevTrack, Semicolon, Slash, Sleep, Stop, Subtract, Sysrq, Underline, Unlabeled, VolumeDown, VolumeUp, Wake, WebBack, WebFavorites, WebForward, WebHome, WebRefresh, WebSearch, WebStop, Yen,
    Copy, Paste, Cut,
}


impl From<GsKeycode> for VirtualKeyCode {
    fn from(code: GsKeycode) -> VirtualKeyCode {
        match code {
            GsKeycode::Key1 => VirtualKeyCode::Key1,
            GsKeycode::Key2 => VirtualKeyCode::Key2,
            GsKeycode::Key3 => VirtualKeyCode::Key3,
            GsKeycode::Key4 => VirtualKeyCode::Key4,
            GsKeycode::Key5 => VirtualKeyCode::Key5,
            GsKeycode::Key6 => VirtualKeyCode::Key6,
            GsKeycode::Key7 => VirtualKeyCode::Key7,
            GsKeycode::Key8 => VirtualKeyCode::Key8,
            GsKeycode::Key9 => VirtualKeyCode::Key9,
            GsKeycode::Key0 => VirtualKeyCode::Key0,
            GsKeycode::A => VirtualKeyCode::A,
            GsKeycode::B => VirtualKeyCode::B,
            GsKeycode::C => VirtualKeyCode::C,
            GsKeycode::D => VirtualKeyCode::D,
            GsKeycode::E => VirtualKeyCode::E,
            GsKeycode::F => VirtualKeyCode::F,
            GsKeycode::G => VirtualKeyCode::G,
            GsKeycode::H => VirtualKeyCode::H,
            GsKeycode::I => VirtualKeyCode::I,
            GsKeycode::J => VirtualKeyCode::J,
            GsKeycode::K => VirtualKeyCode::K,
            GsKeycode::L => VirtualKeyCode::L,
            GsKeycode::M => VirtualKeyCode::M,
            GsKeycode::N => VirtualKeyCode::N,
            GsKeycode::O => VirtualKeyCode::O,
            GsKeycode::P => VirtualKeyCode::P,
            GsKeycode::Q => VirtualKeyCode::Q,
            GsKeycode::R => VirtualKeyCode::R,
            GsKeycode::S => VirtualKeyCode::S,
            GsKeycode::T => VirtualKeyCode::T,
            GsKeycode::U => VirtualKeyCode::U,
            GsKeycode::V => VirtualKeyCode::V,
            GsKeycode::W => VirtualKeyCode::W,
            GsKeycode::X => VirtualKeyCode::X,
            GsKeycode::Y => VirtualKeyCode::Y,
            GsKeycode::Z => VirtualKeyCode::Z,
            GsKeycode::Escape => VirtualKeyCode::Escape,
            GsKeycode::F1  => VirtualKeyCode::F1,
            GsKeycode::F2  => VirtualKeyCode::F2,
            GsKeycode::F3  => VirtualKeyCode::F3,
            GsKeycode::F4  => VirtualKeyCode::F4,
            GsKeycode::F5  => VirtualKeyCode::F5,
            GsKeycode::F6  => VirtualKeyCode::F6,
            GsKeycode::F7  => VirtualKeyCode::F7,
            GsKeycode::F8  => VirtualKeyCode::F8,
            GsKeycode::F9  => VirtualKeyCode::F9,
            GsKeycode::F10 => VirtualKeyCode::F10,
            GsKeycode::F11 => VirtualKeyCode::F11,
            GsKeycode::F12 => VirtualKeyCode::F12,
            GsKeycode::F13 => VirtualKeyCode::F13,
            GsKeycode::F14 => VirtualKeyCode::F14,
            GsKeycode::F15 => VirtualKeyCode::F15,
            GsKeycode::Snapshot   => VirtualKeyCode::Snapshot,
            GsKeycode::Scroll     => VirtualKeyCode::Scroll,
            GsKeycode::Pause      => VirtualKeyCode::Pause,
            GsKeycode::Insert     => VirtualKeyCode::Insert,
            GsKeycode::Home       => VirtualKeyCode::Home,
            GsKeycode::Delete     => VirtualKeyCode::Delete,
            GsKeycode::End        => VirtualKeyCode::End,
            GsKeycode::PageDown   => VirtualKeyCode::PageDown,
            GsKeycode::PageUp     => VirtualKeyCode::PageUp,
            GsKeycode::Left       => VirtualKeyCode::Left,
            GsKeycode::Up         => VirtualKeyCode::Up,
            GsKeycode::Right      => VirtualKeyCode::Right,
            GsKeycode::Down       => VirtualKeyCode::Down,
            GsKeycode::Back       => VirtualKeyCode::Back,
            GsKeycode::Return     => VirtualKeyCode::Return,
            GsKeycode::Space      => VirtualKeyCode::Space,
            GsKeycode::Compose    => VirtualKeyCode::Compose,
            GsKeycode::Caret      => VirtualKeyCode::Caret,
            GsKeycode::Numlock    => VirtualKeyCode::Numlock,
            GsKeycode::Numpad0    => VirtualKeyCode::Numpad0,
            GsKeycode::Numpad1    => VirtualKeyCode::Numpad1,
            GsKeycode::Numpad2    => VirtualKeyCode::Numpad2,
            GsKeycode::Numpad3    => VirtualKeyCode::Numpad3,
            GsKeycode::Numpad4    => VirtualKeyCode::Numpad4,
            GsKeycode::Numpad5    => VirtualKeyCode::Numpad5,
            GsKeycode::Numpad6    => VirtualKeyCode::Numpad6,
            GsKeycode::Numpad7    => VirtualKeyCode::Numpad7,
            GsKeycode::Numpad8    => VirtualKeyCode::Numpad8,
            GsKeycode::Numpad9    => VirtualKeyCode::Numpad9,
            GsKeycode::AbntC1     => VirtualKeyCode::AbntC1,
            GsKeycode::AbntC2     => VirtualKeyCode::AbntC2,
            GsKeycode::Add        => VirtualKeyCode::Add,
            GsKeycode::Apostrophe => VirtualKeyCode::Apostrophe,
            GsKeycode::Apps       => VirtualKeyCode::Apps,
            GsKeycode::At         => VirtualKeyCode::At,
            GsKeycode::Ax         => VirtualKeyCode::Ax,
            GsKeycode::Backslash  => VirtualKeyCode::Backslash,
            GsKeycode::Calculator => VirtualKeyCode::Calculator,
            GsKeycode::Capital    => VirtualKeyCode::Capital,
            GsKeycode::Colon      => VirtualKeyCode::Colon,
            GsKeycode::Comma      => VirtualKeyCode::Comma,
            GsKeycode::Convert    => VirtualKeyCode::Convert,
            GsKeycode::Decimal    => VirtualKeyCode::Decimal,
            GsKeycode::Divide     => VirtualKeyCode::Divide,
            GsKeycode::Equals     => VirtualKeyCode::Equals,
            GsKeycode::Grave      => VirtualKeyCode::Grave,
            GsKeycode::Kana       => VirtualKeyCode::Kana,
            GsKeycode::Kanji      => VirtualKeyCode::Kanji,
            GsKeycode::LAlt       => VirtualKeyCode::LAlt,
            GsKeycode::LBracket   => VirtualKeyCode::LBracket,
            GsKeycode::LControl   => VirtualKeyCode::LControl,
            GsKeycode::LShift     => VirtualKeyCode::LShift,
            GsKeycode::LWin       => VirtualKeyCode::LWin,
            GsKeycode::Mail       => VirtualKeyCode::Mail,
            GsKeycode::MediaSelect      => VirtualKeyCode::MediaSelect,
            GsKeycode::MediaStop        => VirtualKeyCode::MediaStop,
            GsKeycode::Minus            => VirtualKeyCode::Minus,
            GsKeycode::Multiply         => VirtualKeyCode::Multiply,
            GsKeycode::Mute             => VirtualKeyCode::Mute,
            GsKeycode::MyComputer       => VirtualKeyCode::MyComputer,
            GsKeycode::NavigateForward  => VirtualKeyCode::NavigateForward,
            GsKeycode::NavigateBackward => VirtualKeyCode::NavigateBackward,
            GsKeycode::NextTrack        => VirtualKeyCode::NextTrack,
            GsKeycode::NoConvert        => VirtualKeyCode::NoConvert,
            GsKeycode::NumpadComma      => VirtualKeyCode::NumpadComma,
            GsKeycode::NumpadEnter      => VirtualKeyCode::NumpadEnter,
            GsKeycode::NumpadEquals     => VirtualKeyCode::NumpadEquals,
            GsKeycode::OEM102           => VirtualKeyCode::OEM102,
            GsKeycode::Period           => VirtualKeyCode::Period,
            GsKeycode::PlayPause        => VirtualKeyCode::PlayPause,
            GsKeycode::Power            => VirtualKeyCode::Power,
            GsKeycode::PrevTrack        => VirtualKeyCode::PrevTrack,
            GsKeycode::RAlt             => VirtualKeyCode::RAlt,
            GsKeycode::RBracket         => VirtualKeyCode::RBracket,
            GsKeycode::RControl         => VirtualKeyCode::RControl,
            GsKeycode::RShift           => VirtualKeyCode::RShift,
            GsKeycode::RWin             => VirtualKeyCode::RWin,
            GsKeycode::Semicolon        => VirtualKeyCode::Semicolon,
            GsKeycode::Slash            => VirtualKeyCode::Slash,
            GsKeycode::Sleep            => VirtualKeyCode::Sleep,
            GsKeycode::Stop             => VirtualKeyCode::Stop,
            GsKeycode::Subtract         => VirtualKeyCode::Subtract,
            GsKeycode::Sysrq            => VirtualKeyCode::Sysrq,
            GsKeycode::Tab              => VirtualKeyCode::Tab,
            GsKeycode::Underline        => VirtualKeyCode::Underline,
            GsKeycode::Unlabeled        => VirtualKeyCode::Unlabeled,
            GsKeycode::VolumeDown       => VirtualKeyCode::VolumeDown,
            GsKeycode::VolumeUp         => VirtualKeyCode::VolumeUp,
            GsKeycode::Wake             => VirtualKeyCode::Wake,
            GsKeycode::WebBack          => VirtualKeyCode::WebBack,
            GsKeycode::WebFavorites     => VirtualKeyCode::WebFavorites,
            GsKeycode::WebForward       => VirtualKeyCode::WebForward,
            GsKeycode::WebHome          => VirtualKeyCode::WebHome,
            GsKeycode::WebRefresh       => VirtualKeyCode::WebRefresh,
            GsKeycode::WebSearch        => VirtualKeyCode::WebSearch,
            GsKeycode::WebStop          => VirtualKeyCode::WebStop,
            GsKeycode::Yen              => VirtualKeyCode::Yen,
            GsKeycode::Copy             => VirtualKeyCode::Copy,
            GsKeycode::Paste            => VirtualKeyCode::Paste,
            GsKeycode::Cut              => VirtualKeyCode::Cut,
        }
    }
}
