#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, felt, Felt, StorageMap, Word};

#[component]
struct LevelAccount {
    #[storage(description = "level account xp storage")]
    xp: StorageMap<Word, Felt>,
}

#[component]
impl LevelAccount {
    pub fn get_xp(&self) -> Felt {
        let key = Word::new([felt!(21), felt!(0), felt!(0), felt!(0)]);
        self.xp.get(key)
    }

    pub fn add_xp(&mut self) -> Felt {
        let key = Word::new([felt!(21), felt!(0), felt!(0), felt!(0)]);
        let current = self.xp.get(key);
        let new_value = current + felt!(25);
        self.xp.set(key, new_value);
        new_value
    }

    pub fn reset_xp(&mut self) {
        let key = Word::new([felt!(21), felt!(0), felt!(0), felt!(0)]);
        self.xp.set(key, felt!(0));
    }
}
