use raylib::prelude::{GamepadButton, KeyboardKey};

use crate::MAX_ENTITIES;
#[derive(Debug, Clone, Copy)]
pub struct PlayerInput {
    pub keybard: [KbConfig; MAX_ENTITIES],
    pub gamepad: [PadConfig; MAX_ENTITIES],
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            keybard: [KbConfig::p1_keys(), KbConfig::p2_keys()],
            gamepad: Default::default(),
        }
    }

}

#[derive(Debug, Clone, Copy)]
pub struct KbConfig {
    pub up: KeyboardKey,
    pub down: KeyboardKey,
    pub back: KeyboardKey,
    pub forward: KeyboardKey,
    pub a: KeyboardKey,
    pub b: KeyboardKey,
    pub c: KeyboardKey,
    pub k: KeyboardKey,
}

impl KbConfig {
    pub fn p1_keys() -> Self {
        Self {
            up: KeyboardKey::KEY_W,
            down: KeyboardKey::KEY_S,
            back: KeyboardKey::KEY_A,
            forward: KeyboardKey::KEY_D,
            a: KeyboardKey::KEY_J,
            b: KeyboardKey::KEY_K,
            c: KeyboardKey::KEY_L,
            k: KeyboardKey::KEY_N,
        }
    }

    pub fn p2_keys() -> Self {
        Self {
            up: KeyboardKey::KEY_UP,
            down: KeyboardKey::KEY_DOWN,
            back: KeyboardKey::KEY_LEFT,
            forward: KeyboardKey::KEY_RIGHT,
            a: KeyboardKey::KEY_V,
            b: KeyboardKey::KEY_C,
            c: KeyboardKey::KEY_X,
            k: KeyboardKey::KEY_Z,
        }
    }
}

impl Default for KbConfig {
    fn default() -> Self {
        Self {
            up: KeyboardKey::KEY_W,
            down: KeyboardKey::KEY_S,
            back: KeyboardKey::KEY_A,
            forward: KeyboardKey::KEY_D,
            a: KeyboardKey::KEY_J,
            b: KeyboardKey::KEY_K,
            c: KeyboardKey::KEY_L,
            k: KeyboardKey::KEY_N,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PadConfig {
    pub up: GamepadButton,
    pub down: GamepadButton,
    pub back: GamepadButton,
    pub forward: GamepadButton,
    pub a: GamepadButton,
    pub b: GamepadButton,
    pub c: GamepadButton,
    pub k: GamepadButton,
}

impl Default for PadConfig {
    fn default() -> Self {
        Self {
            up: GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP,
            down: GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN,
            back: GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT,
            forward: GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT,
            a: GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT,
            b: GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP,
            c: GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT,
            k: GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN,
        }
    }
}
