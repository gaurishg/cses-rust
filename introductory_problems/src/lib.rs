pub mod p01_weird_algorithm;
pub mod p02_missing_number;
pub mod p03_repetitions;
pub mod p04_increasing_array;
pub mod p05_permutations;
pub mod p08_two_sets;
pub mod p09_bit_strings;
pub mod p10_trailing_zeros;
pub mod p11_coin_piles;
pub mod p12_palindrome_reorder;
pub mod p13_gray_code;
pub mod p14_tower_of_hanoi;
pub mod p15_creating_strings;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
