use test_demo;
mod common;
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_demo::add_two(2));
}
