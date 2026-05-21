#![no_std]
#![feature(alloc_error_handler)]

use miden::{component, felt, Felt, StorageMap, Word};

#[component]
struct PointsAccount {
    #[storage(description = "points account storage")]
    points: StorageMap<Word, Felt>,
}

#[component]
impl PointsAccount {
    pub fn get_points(&self) -> Felt {
        let key = Word::new([felt!(9), felt!(0), felt!(0), felt!(0)]);
        self.points.get(key)
    }

    pub fn add_points(&mut self) -> Felt {
        let key = Word::new([felt!(9), felt!(0), felt!(0), felt!(0)]);
        let current = self.points.get(key);
        let new_value = current + felt!(10);
        self.points.set(key, new_value);
        new_value
    }

    pub fn reset_points(&mut self) {
        let key = Word::new([felt!(9), felt!(0), felt!(0), felt!(0)]);
        self.points.set(key, felt!(0));
    }
}

