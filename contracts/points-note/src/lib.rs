#![no_std]
#![feature(alloc_error_handler)]

use miden::*;

use crate::bindings::miden::points_account::points_account;

#[note]
struct PointsNote;

#[note]
impl PointsNote {
    #[note_script]
    fn run(self, _arg: Word) {
        let initial = points_account::get_points();
        points_account::add_points();
        let final_value = points_account::get_points();

        assert_eq(final_value, initial + Felt::from_u32(10));
    }
}
