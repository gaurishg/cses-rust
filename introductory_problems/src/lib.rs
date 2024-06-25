pub mod p01_weird_algorithm;
pub mod p02_missing_number;
pub mod p03_repetitions;
pub mod p04_increasing_array;
pub mod p05_permutations;
pub mod p08_two_sets;

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
