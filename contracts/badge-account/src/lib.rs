#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, felt, Felt, StorageMap, Word};

#[component]
struct BadgeAccount {
    #[storage(description = "badge account storage")]
    badges: StorageMap<Word, Felt>,
}

#[component]
impl BadgeAccount {
    pub fn get_badges(&self) -> Felt {
        let key = Word::new([felt!(11), felt!(0), felt!(0), felt!(0)]);
        self.badges.get(key)
    }

    pub fn add_badge(&mut self) -> Felt {
        let key = Word::new([felt!(11), felt!(0), felt!(0), felt!(0)]);
        let current = self.badges.get(key);
        let new_value = current + felt!(1);
        self.badges.set(key, new_value);
        new_value
    }

    pub fn reset_badges(&mut self) {
        let key = Word::new([felt!(11), felt!(0), felt!(0), felt!(0)]);
        self.badges.set(key, felt!(0));
    }
}
