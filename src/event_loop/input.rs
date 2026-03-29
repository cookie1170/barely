use std::collections::HashSet;

use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct InputState {
    pressed: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,
    just_released: HashSet<KeyCode>,
}

impl InputState {
    pub fn pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn released(&self, key: KeyCode) -> bool {
        !self.pressed.contains(&key)
    }

    pub fn just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn just_released(&self, key: KeyCode) -> bool {
        self.just_released.contains(&key)
    }
}

impl InputState {
    pub(super) fn on_pressed(&mut self, key: KeyCode) {
        self.pressed.insert(key);
        self.just_pressed.insert(key);
    }

    pub(super) fn on_released(&mut self, key: KeyCode) {
        self.pressed.remove(&key);
        self.just_released.insert(key);
    }

    pub(super) fn on_update(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub(super) fn on_focus_lost(&mut self) {
        // if the window loses focus, we want to count all keys that are currently pressed as getting released
        // this only automatically works on windows and x11, so we need to do it manually
        for key in self.pressed.iter().cloned().collect::<Vec<_>>() {
            self.on_released(key);
        }
    }
}

impl<'a> Context<'a> {
    pub fn input(&self) -> &InputState {
        self.input_state
    }
}

impl<'a> FixedContext<'a> {
    pub fn input(&self) -> &InputState {
        self.input_state
    }
}
