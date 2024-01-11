use rb11_tests;
use rb11_tests::Rectangle;


#[test]
fn cannot_hold_other_of_equal_size() {
    let one = Rectangle {
        width: 5,
        height: 7,
    };
    let two = Rectangle {
        width: 5,
        height: 7,
    };

    assert!(!one.can_hold(&two));
}