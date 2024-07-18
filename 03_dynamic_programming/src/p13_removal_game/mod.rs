use std::io::Read;

fn f(v: Vec<isize>) -> isize {
    let n = v.len();
    let cumsum = v
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<isize>>();

    let get_sum = |l, r| {
        if r >= cumsum.len() || l > r {
            0
        } else if l == 0 {
            cumsum[r]
        } else {
            cumsum[r] - cumsum[l - 1]
        }
    };

    let mut dp = vec![vec![0; n + 1]; n + 1];
    // f(a, b) => max(v[a] + sum(a+1, b) - f(a+1, b), v[b] + sum(a, b-1) + f(a, b-1))
    for a in (0..n).rev() {
        for b in a..n {
            if a == b {
                dp[a][b] = v[a];
                continue;
            }
            dp[a][b] = std::cmp::max(
                v[a] + get_sum(a + 1, b) - dp[a + 1][b],
                v[b] + get_sum(a, b - 1) - dp[a][b - 1],
            );
        }
    }

    dp[0][n - 1]
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    println!(
        "{}",
        f(s.lines()
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect())
    );
}

#[cfg(test)]
mod test_removal_game {
    use super::f;

    #[test]
    fn example() {
        assert_eq!(8, f(vec![4, 5, 1, 3]));
    }
}
