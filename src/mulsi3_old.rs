pub fn mulsi3(a: u32, b: u32) -> u32 {
    let (mut a, mut b) = (a, b);
    let mut r = 0;

    while a > 0 {
        if a & 1 > 0 {
            r += b;
        }
        a >>= 1;
        b <<= 1;
    }

    r
}
