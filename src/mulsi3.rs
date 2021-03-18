pub fn mulsi3(a: u32, b: u32) -> u32 {
    let mut sum = 0;

    for i in 0..32 {
        let bit = b & (1 << i);
        if bit != 0 {
            sum += a << i;
        }
    }

    sum
}
