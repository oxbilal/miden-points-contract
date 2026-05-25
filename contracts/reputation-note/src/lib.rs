#![no_std]
#![feature(alloc_error_handler)]

use miden::*;

use crate::bindings::miden::reputation_account::reputation_account;

#[note]
struct ReputationNote;

#[note]
impl ReputationNote {
    #[note_script]
    fn run(self, _arg: Word) {
        let initial = reputation_account::get_reputation();
        reputation_account::add_reputation();
        let final_value = reputation_account::get_reputation();

        assert_eq(final_value, initial + Felt::from_u32(50));
    }
}
