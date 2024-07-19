use std::io::Read;

fn f(v: Vec<usize>) -> usize {
    let mut lengths = vec![];
    for x in v {
        match lengths.binary_search(&x) {
            Ok(pos) | Err(pos) => {
                if pos == lengths.len() {
                    lengths.push(x);
                } else {
                    lengths[pos] = x;
                }
            }
        }
    }

    lengths.len()
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);
    println!(
        "{}",
        f(s.split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect())
    );
}

#[cfg(test)]
mod test_increasing_subsequence {
    use super::f;

    #[test]
    fn example() {
        assert_eq!(f(vec![7, 3, 5, 3, 6, 2, 9, 8]), 4);
    }
}
