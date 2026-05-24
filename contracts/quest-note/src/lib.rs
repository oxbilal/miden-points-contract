#![no_std]
#![feature(alloc_error_handler)]

use miden::*;

use crate::bindings::miden::quest_account::quest_account;

#[note]
struct QuestNote;

#[note]
impl QuestNote {
    #[note_script]
    fn run(self, _arg: Word) {
        let initial = quest_account::get_quests();
        quest_account::complete_quest();
        let final_value = quest_account::get_quests();

        assert_eq(final_value, initial + Felt::from_u32(1));
    }
}
