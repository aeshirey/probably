#![crate_name = "probably"]
#![warn(non_camel_case_types, non_upper_case_globals, unused_qualifications)]
#![allow(non_snake_case)]
#![allow(clippy::unreadable_literal)]

pub mod counting;
pub mod membership;

#[cfg(test)]
mod tests {
    #[test]
    fn hll_test() {
        let mut my_hll = crate::counting::hll::HyperLogLog::new(0.01);
        my_hll.insert(&"foobar".to_string());
        my_hll.insert(&"baz");
        my_hll.insert(&1234_u64);
        my_hll.insert(&true);

        let my_hll_len = my_hll.len() as u64;
        assert_eq!(my_hll_len, 4);
    }
}
