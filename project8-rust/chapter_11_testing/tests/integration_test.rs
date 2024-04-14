mod common;

use chapter_11_testing::*;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, add_two(2));
}