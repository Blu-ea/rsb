// Shader exemple of Hilbert curve : https://www.shadertoy.com/view/XtGBDW

use std::mem::swap;

pub fn map(x: u16, y: u16) -> f64{
    let mut index: u64 = 0;
    let n = 16;
    let mut s = 1 << (n - 1);
    let mut x: u32 = x as u32;
    let mut y: u32 = y as u32;

    while s > 0{
        let rx = ((x & s) != 0) as u32;
        let ry = ((y & s) != 0) as u32;

        index += (s as u64 * s as u64) * ((3 * rx) ^ ry) as u64;

        if ry == 0 {
            if rx == 1{
                x = (1 << n) - 1 - x;
                y = (1 << n) - 1 - y;
            }
            swap(&mut x, &mut y);
        }
        s >>= 1;
    }
    index as f64 / (1u64 << (2 * n)) as f64
}