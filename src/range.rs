pub fn foo(out: &mut Vec<usize>, inx: &[usize], base: usize) {
    for (i, item) in out[..base].iter_mut().enumerate().skip(1) {
        *item = *inx.get(i).unwrap_or(&0);
    }
}
