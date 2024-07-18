use std::io::Read;

fn f(s1: &str, s2: &str) -> usize {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    for r in 0..s1.len() {
        *dp[r].last_mut().unwrap() = s1.len() - r;
    }

    for c in 0..s2.len() {
        dp.last_mut().unwrap()[c] = s2.len() - c;
    }

    for r in (0..s1.len()).rev() {
        for c in (0..s2.len()).rev() {
            if s1[r] == s2[c] {
                dp[r][c] = dp[r + 1][c + 1];
            } else {
                dp[r][c] = *[
                    1 + dp[r + 1][c + 1], // replace
                    1 + dp[r][c + 1],     // add
                    1 + dp[r + 1][c],
                ]
                .iter()
                .min()
                .unwrap()
            }
        }
    }

    dp[0][0]
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);
    println!(
        "{}",
        f(s.lines().nth(0).unwrap(), s.lines().nth(1).unwrap())
    );
}

#[cfg(test)]
mod test_edit_distance {
    use super::f;

    #[test]
    fn example() {
        assert_eq!(2, f("LOVE", "MOVIE"));
    }

    #[test]
    fn test_1() {
        assert_eq!(8, f("NEABJPJOI", "RFMQRJKJKIA"));
    }
}
