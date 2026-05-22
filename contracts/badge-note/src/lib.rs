#![no_std]
#![feature(alloc_error_handler)]

use miden::*;

use crate::bindings::miden::badge_account::badge_account;

#[note]
struct BadgeNote;

#[note]
impl BadgeNote {
    #[note_script]
    fn run(self, _arg: Word) {
        let initial = badge_account::get_badges();
        badge_account::add_badge();
        let final_value = badge_account::get_badges();

        assert_eq(final_value, initial + Felt::from_u32(1));
    }
}
