pub fn gray_code(n: u32) -> u32{
    n ^ (n >> 1)
}