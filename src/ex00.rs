
pub fn adder(mut a: u32, mut b: u32) -> u32 {
    let mut carry : u32;
    while b != 0 {
        carry =  a & b; // add without the carry;
        a = a ^ b;      // calculate the carry number;
        b = carry << 1; // i move the carry number to the side and prepare to the next addition;
    }

    a
}
