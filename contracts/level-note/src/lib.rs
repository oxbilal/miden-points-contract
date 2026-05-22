#![no_std]
#![feature(alloc_error_handler)]

use miden::*;

use crate::bindings::miden::level_account::level_account;

#[note]
struct LevelNote;

#[note]
impl LevelNote {
    #[note_script]
    fn run(self, _arg: Word) {
        let initial = level_account::get_xp();
        level_account::add_xp();
        let final_value = level_account::get_xp();

        assert_eq(final_value, initial + Felt::from_u32(25));
    }
}
