macro_rules! to {
    ($e:expr, $ty:ty) => {
        $e as $ty
    };
}

const L: usize = 9;
const BAD: [bool; 42] = [true; 42];

fn reverse_digits(_i: u32, digs: &mut Vec<u32>) -> Vec<u32> {
    digs.clone()
}

pub fn foo(primes: &mut Vec<bool>, digs: &mut Vec<u32>) {
    for (i, pi) in primes.iter_mut().enumerate().take(L).skip(6) {
        if *pi
            && reverse_digits(to! {i, u32}, digs)
                .iter()
                .any(|&d| BAD[to! {d, usize}])
        {
            *pi = false;
        }
    }
}
