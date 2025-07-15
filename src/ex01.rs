use crate::ex00::adder;

pub fn multiplier(mut a: u32, mut b: u32) -> u32{
    let mut result = 0;
    while b > 0{
        if b & 1 == 1{ // if b is odd, we add a to the result;
            result = adder(result, a);
        }

        a <<= 1; // everytime, we multiply a by 2
        b >>= 1; // and divide b by 2
    }

    result
}