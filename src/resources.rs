use std::collections::HashMap;

use specs::BitSet;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum LabelVariable {
    HeroLives,
    FramesPerSecond,
    Score,
    EngineVersion,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum GameStateFlag {
    Victory = 1,
    Defeat = 2,
}

#[derive(Default, Copy, Clone)]
pub struct GameStateFlagRes {
    pub flag: Option<GameStateFlag>,
}

#[derive(Default)]
pub struct VariableDictionary {
    pub dictionary: HashMap<LabelVariable, String>,
}

pub enum KeyboardKeys {
    KeyUp = 1,
    KeyLeft = 2,
    KeyRight = 4,
}

#[derive(Default)]
pub struct PressedKeys {
    pub pressed_keys: BitSet,
}
