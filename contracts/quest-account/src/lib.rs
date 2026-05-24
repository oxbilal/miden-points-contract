#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, felt, Felt, StorageMap, Word};

#[component]
struct QuestAccount {
    #[storage(description = "quest account storage")]
    quests: StorageMap<Word, Felt>,
}

#[component]
impl QuestAccount {
    pub fn get_quests(&self) -> Felt {
        let key = Word::new([felt!(31), felt!(0), felt!(0), felt!(0)]);
        self.quests.get(key)
    }

    pub fn complete_quest(&mut self) -> Felt {
        let key = Word::new([felt!(31), felt!(0), felt!(0), felt!(0)]);
        let current = self.quests.get(key);
        let new_value = current + felt!(1);
        self.quests.set(key, new_value);
        new_value
    }

    pub fn reset_quests(&mut self) {
        let key = Word::new([felt!(31), felt!(0), felt!(0), felt!(0)]);
        self.quests.set(key, felt!(0));
    }
}
