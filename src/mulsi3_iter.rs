pub fn mulsi3(a: u32, b: u32) -> u32 {
    (0..32).fold(0, |sum, i| {
        let bit = b & (1 << i);
        if bit != 0 {
            sum + (a << i)
        } else {
            sum
        }
    })
}
