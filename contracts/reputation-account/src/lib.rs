#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, felt, Felt, StorageMap, Word};

#[component]
struct ReputationAccount {
    #[storage(description = "reputation score storage")]
    reputation: StorageMap<Word, Felt>,
}

#[component]
impl ReputationAccount {
    pub fn get_reputation(&self) -> Felt {
        let key = Word::new([felt!(41), felt!(0), felt!(0), felt!(0)]);
        self.reputation.get(key)
    }

    pub fn add_reputation(&mut self) -> Felt {
        let key = Word::new([felt!(41), felt!(0), felt!(0), felt!(0)]);
        let current = self.reputation.get(key);
        let new_value = current + felt!(50);
        self.reputation.set(key, new_value);
        new_value
    }

    pub fn reset_reputation(&mut self) {
        let key = Word::new([felt!(41), felt!(0), felt!(0), felt!(0)]);
        self.reputation.set(key, felt!(0));
    }
}
