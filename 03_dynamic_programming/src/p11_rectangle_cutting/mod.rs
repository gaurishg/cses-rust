use std::io::Read;

fn f(a: usize, b: usize) -> usize {
    let mut dp = vec![vec![usize::MAX; b + 1]; a + 1];

    for c in 1..=b {
        dp[1][c] = c - 1;
    }

    for r in 1..=a {
        dp[r][1] = r - 1;
    }

    for r in 2..=a {
        for c in 2..=b {
            if r == c {
                dp[r][c] = 0;
            }
            for row_cut in 1..r {
                dp[r][c] = std::cmp::min(dp[r][c], 1 + dp[row_cut][c] + dp[r - row_cut][c]);
            }
            for col_cut in 1..c {
                dp[r][c] = std::cmp::min(dp[r][c], 1 + dp[r][col_cut] + dp[r][c - col_cut]);
            }
        }
    }

    dp[a][b]
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    println!(
        "{}",
        f(
            s.split_whitespace().nth(0).unwrap().parse().unwrap(),
            s.split_whitespace().nth(1).unwrap().parse().unwrap()
        )
    );
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn example() {
        assert_eq!(3, f(3, 5));
    }

    #[test]
    #[ignore]
    fn test_6() {
        assert_eq!(10, f(404, 288));
    }

    #[test]
    #[ignore]
    fn test_7() {
        assert_eq!(13, f(349, 234));
    }
}
