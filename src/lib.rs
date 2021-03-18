pub mod index;
pub mod index2;
pub mod iter;
pub mod iter2;
pub mod mulsi3;
pub mod mulsi3_iter;
pub mod mulsi3_old;
pub mod range;

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 100000, .. ProptestConfig::default()
        })]
        #[test]
        fn mulsi3_eq_mulsi3_iter(a: u32, b: u32) {
            prop_assert_eq!(mulsi3::mulsi3(a, b), mulsi3_iter::mulsi3(a, b));
        }
        #[test]
        fn mulsi3_eq_mulsi3_old(a: u32, b: u32) {
            prop_assert_eq!(mulsi3::mulsi3(a, b), mulsi3_old::mulsi3(a, b));
        }
        #[test]
        fn mulsi3_iter_eq_mulsi3_old(a: u32, b: u32) {
            prop_assert_eq!(mulsi3_iter::mulsi3(a, b), mulsi3_old::mulsi3(a, b));
        }
    }
}
