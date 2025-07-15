use rsb::*;

fn assert_multiplier_eq (a : u32, b : u32) {
    assert_eq!(ex01::multiplier(a, b), a * b);
}

#[test]
fn test_ex01() {
    assert_multiplier_eq(1, 2);
    assert_multiplier_eq(2, 2);
    assert_multiplier_eq(0, 2);
    assert_multiplier_eq(2, 0);
    assert_multiplier_eq(3, 17);
    assert_multiplier_eq(50, 50);
    assert_multiplier_eq(50, 9);
    assert_multiplier_eq(1, 9);
}