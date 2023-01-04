use std::fs;
use bevy::{
    prelude::*,
    window::WindowMode,
};
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components_events::{Hotkeys, Action};

pub fn load_settings(
    mut hotkey_event: EventWriter<Hotkeys>,
    mut windows: ResMut<Windows>,
) {
    #[derive(Debug, Serialize, Deserialize)]
    struct JsonSettings {
        window_type: String,
        resolution: [f32; 2],
        master: f32,
        characters: f32,
        environment: f32,
        keybinds: [[String; 2]; 7]
    }

    let string_data = fs::read_to_string("assets/settings.json").unwrap();

    println!("string_data: {}", string_data);
    let json_settings: JsonSettings = serde_json::from_str(&string_data).unwrap();
    let mut input_map: InputMap<Action> = InputMap::default();

    match json_settings.window_type.as_str() {
        
        "BorderlessFullscreen" => {
            for window in windows.iter_mut() {
                window.set_mode(WindowMode::BorderlessFullscreen);
                window.set_resizable(false);
                window.set_title(String::from("Project Bones"));
            }
        }
        
        "Fullscreen" => {
            for window in windows.iter_mut() {
                window.set_mode(WindowMode::Fullscreen);
                window.set_resizable(false);
                window.set_title(String::from("Project Bones"));
            }
        }

        "Windowed" => {
            for window in windows.iter_mut() {
                window.set_mode(WindowMode::Windowed);
                window.set_resizable(true);
                window.set_title(String::from("Project Bones"));
            }
        }

        _ => {
            for window in windows.iter_mut() {
                window.set_mode(WindowMode::BorderlessFullscreen);
                window.set_resizable(false);
                window.set_title(String::from("Project Bones"));
            }
        }
    }

    for window in windows.iter_mut() {
        window.set_resolution(json_settings.resolution[0], json_settings.resolution[1]);
    }

    for keybind in json_settings.keybinds.iter() {
        input_map.insert(string_to_keycode(&keybind[1]), string_to_action(&keybind[0]));
    }

    hotkey_event.send(Hotkeys(input_map));
}

fn string_to_keycode(string: &str) -> KeyCode {
    match string {
        "1" => KeyCode::Key1,
        "2" => KeyCode::Key2,
        "3" => KeyCode::Key3,
        "4" => KeyCode::Key4,
        "5" => KeyCode::Key5,
        "6" => KeyCode::Key6,
        "7" => KeyCode::Key7,
        "8" => KeyCode::Key8,
        "9" => KeyCode::Key9,
        "0" => KeyCode::Key0,

        "a" => KeyCode::A,
        "b" => KeyCode::B,
        "c" => KeyCode::C,
        "d" => KeyCode::D,
        "e" => KeyCode::E,
        "f" => KeyCode::F,
        "g" => KeyCode::G,
        "h" => KeyCode::H,
        "i" => KeyCode::I,
        "j" => KeyCode::J,
        "k" => KeyCode::K,
        "l" => KeyCode::L,
        "m" => KeyCode::M,
        "n" => KeyCode::N,
        "o" => KeyCode::O,
        "p" => KeyCode::P,
        "q" => KeyCode::Q,
        "r" => KeyCode::R,
        "s" => KeyCode::S,
        "t" => KeyCode::T,
        "u" => KeyCode::U,
        "v" => KeyCode::V,
        "w" => KeyCode::W,
        "x" => KeyCode::X,
        "y" => KeyCode::Y,
        "z" => KeyCode::Z,

        "escape" => KeyCode::Escape,

        "f1" => KeyCode::F1,
        "f2" => KeyCode::F2,
        "f3" => KeyCode::F3,
        "f4" => KeyCode::F4,
        "f5" => KeyCode::F5,
        "f6" => KeyCode::F6,
        "f7" => KeyCode::F7,
        "f8" => KeyCode::F8,
        "f9" => KeyCode::F9,
        "f10" => KeyCode::F10,
        "f11" => KeyCode::F11,
        "f12" => KeyCode::F12,
        "f13" => KeyCode::F13,
        "f14" => KeyCode::F14,
        "f15" => KeyCode::F15,
        "f16" => KeyCode::F16,
        "f17" => KeyCode::F17,
        "f18" => KeyCode::F18,
        "f19" => KeyCode::F19,
        "f20" => KeyCode::F20,
        "f21" => KeyCode::F21,
        "f22" => KeyCode::F22,
        "f23" => KeyCode::F23,
        "f24" => KeyCode::F24,

        "snapshot" => KeyCode::Snapshot,
        "scroll" => KeyCode::Scroll,
        "pause" => KeyCode::Pause,

        "insert" => KeyCode::Insert,
        "home" => KeyCode::Home,
        "delete" => KeyCode::Delete,
        "end" => KeyCode::End,
        "pagedown" => KeyCode::PageDown,
        "pageup" => KeyCode::PageUp,

        "left" => KeyCode::Left,
        "up" => KeyCode::Up,
        "right" => KeyCode::Right,
        "down" => KeyCode::Down,
        "backspace" => KeyCode::Back,
        "return" => KeyCode::Return,
        "spacebar" => KeyCode::Space,

        "compose" => KeyCode::Compose,
        "caret" => KeyCode::Caret,

        "numlock" => KeyCode::Numlock,
        "numpad0" => KeyCode::Numpad0,
        "numpad1" => KeyCode::Numpad1,
        "numpad2" => KeyCode::Numpad2,
        "numpad3" => KeyCode::Numpad3,
        "numpad4" => KeyCode::Numpad4,
        "numpad5" => KeyCode::Numpad5,
        "numpad6" => KeyCode::Numpad6,
        "numpad7" => KeyCode::Numpad7,
        "numpad8" => KeyCode::Numpad8,
        "numpad9" => KeyCode::Numpad9,

        "abntc1" => KeyCode::AbntC1,
        "abntc2" => KeyCode::AbntC2,

        "numpadadd" => KeyCode::NumpadAdd,
        "apostrophe" => KeyCode::Apostrophe,
        "apps" => KeyCode::Apps,
        "asterisk" => KeyCode::Asterisk,
        "plus" => KeyCode::Plus,
        "at" => KeyCode::At,
        "ax" => KeyCode::Ax,
        "backslash" => KeyCode::Backslash,
        "calculator" => KeyCode::Calculator,
        "capitol" => KeyCode::Capital,
        "colon" => KeyCode::Colon,
        "comma" => KeyCode::Comma,
        "convert" => KeyCode::Convert,
        "numpaddecimal" => KeyCode::NumpadDecimal,
        "numpaddivide" => KeyCode::NumpadDivide,
        "equals" => KeyCode::Equals,
        "grave" => KeyCode::Grave,
        "kana" => KeyCode::Kana,
        "kanji" => KeyCode::Kanji,

        "leftalt" => KeyCode::LAlt,
        "leftbracket" => KeyCode::LBracket,
        "leftcontrol" => KeyCode::LControl,
        "leftshift" => KeyCode::LShift,
        "leftwin" => KeyCode::LWin,

        "mail" => KeyCode::Mail,
        "mediaselect" => KeyCode::MediaSelect,
        "mediastop" => KeyCode::MediaStop,
        "minus" => KeyCode::Minus,
        "numpadmultiply" => KeyCode::NumpadMultiply,
        "mute" => KeyCode::Mute,
        "mycomputer" => KeyCode::MyComputer,
        "navigateforward" => KeyCode::NavigateForward,
        "navigatebackward" => KeyCode::NavigateBackward,
        "nexttrack" => KeyCode::NextTrack,
        "noconvert" => KeyCode::NoConvert,
        "numpadcomma" => KeyCode::NumpadComma,
        "numpadenter" => KeyCode::NumpadEnter,
        "numpadequals" => KeyCode::NumpadEquals,
        "oem102" => KeyCode::Oem102,
        "period" => KeyCode::Period,
        "playpause" => KeyCode:: PlayPause,
        "power" => KeyCode::Power,
        "prevtrack" => KeyCode::PrevTrack,

        "rightalt" => KeyCode::RAlt,
        "rightbacket" => KeyCode::RBracket,
        "rightcontrol" => KeyCode::RControl,
        "rightshift" => KeyCode::RShift,
        "rightwindows" => KeyCode::RWin,

        "semicolon" => KeyCode::Semicolon,
        "slash" => KeyCode:: Slash,
        "sleep" => KeyCode::Sleep,
        "stop" => KeyCode::Stop,
        "numpadsubtract" => KeyCode::NumpadSubtract,
        "sysrq" => KeyCode::Sysrq,
        "tab" => KeyCode::Tab,
        "underline" => KeyCode::Underline,
        "unlabeled" => KeyCode::Unlabeled,

        "volumedown" => KeyCode::VolumeDown,
        "volumeup" => KeyCode::VolumeUp,

        "wake" => KeyCode::Wake,

        "webback" => KeyCode::WebBack,
        "webfavorites" => KeyCode::WebFavorites,
        "webforward" => KeyCode::WebForward,
        "webhome" => KeyCode::WebHome,
        "webrefresh" => KeyCode::WebRefresh,
        "websearch" => KeyCode::WebSearch,
        "webstop" => KeyCode::WebStop,

        "yen" => KeyCode::Yen,

        "copy" => KeyCode:: Copy,
        "paste" => KeyCode::Paste,
        "key" => KeyCode::Cut,
        
        _ => KeyCode::Space,
    }
}

fn string_to_action(string: &str) -> Action {
    match string {
        "Up" => Action::Up,
        "Down" => Action::Down,
        "Left" => Action::Left,
        "Right" => Action::Right,
        "Split" => Action::Split,
        
        "Interact" => Action::Interact,
        "CycleForward" => Action::CycleForward,
        "CycleBackward" => Action::CycleBackward,
        
        _ => Action::Unused,
    }
}