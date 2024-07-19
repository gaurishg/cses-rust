use std::io::Read;

fn f(v: Vec<usize>) -> usize {
    let n = v.len();
    let mut dp = vec![1; n];
    let mut longest = 1;
    for i in 1..n {
        for j in 0..i {
            if v[j] < v[i] {
                dp[i] = std::cmp::max(dp[i], 1 + dp[j]);
            }
        }
        longest = std::cmp::max(longest, dp[i]);
    }

    longest
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
