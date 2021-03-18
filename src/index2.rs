pub fn foo(out: &mut Vec<usize>, inx: &[usize], base: usize) {
    for i in 1..base {
        out[i] = *inx.get(i).unwrap_or(&0);
    }
}
