use rsb::*;

fn assert_adder_eq (a : u32, b : u32) {
    assert_eq!(ex00::adder(a, b), a + b);
}

#[test]
fn test_ex00() {
    assert_adder_eq(1, 2);
    assert_adder_eq(2, 2);
    assert_adder_eq(0, 2);
    assert_adder_eq(2, 0);
    assert_adder_eq(50, 50);
    assert_adder_eq(50, 9);
    assert_adder_eq(1, 9);
}