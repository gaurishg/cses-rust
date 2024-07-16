use std::{collections::HashMap, io::Read};

fn calculate(
    upper_bound: usize,
    v: &Vec<usize>,
    i: usize,
    vi: usize,
    dp: &mut HashMap<usize, HashMap<usize, usize>>,
) -> usize {
    if v[i] != 0 && vi != v[i] || vi == 0 || vi > upper_bound {
        return 0;
    }

    if i == v.len() - 1 {
        return 1;
    }

    if dp.entry(i).or_insert(HashMap::new()).get(&vi).is_some() {
        return dp[&i][&vi];
    }

    for x in [vi - 1, vi, vi + 1] {
        if x == 0 || x > upper_bound {
            continue;
        }
        let cal = calculate(upper_bound, v, i + 1, x, dp);
        dp.entry(i).or_insert(HashMap::new()).entry(vi).or_default(); // enter 0
                                                                      // if does not exists

        dp.entry(i)
            .or_insert(HashMap::new())
            .entry(vi)
            .and_modify(|val| *val = (*val + cal) % 1_000_000_007)
            .or_default();
    }
    return dp[&i][&vi];
}

fn f(upper_bound: usize, v: Vec<usize>) -> usize {
    let n = v.len();
    let mut dp = vec![vec![0; n]; upper_bound + 2];

    for m in 1..=upper_bound {
        if v[n - 1] != 0 {
            if m == v[n - 1] {
                dp[m][n - 1] = 1;
            }
        } else {
            dp[m][n - 1] = 1;
        }
    }

    for col in (0..n - 1).rev() {
        for row in 1..=upper_bound {
            if v[col] != 0 {
                if row == v[col] {
                    dp[row][col] = (dp[row - 1][col + 1] + dp[row][col + 1] + dp[row + 1][col + 1])
                        % 1_000_000_007;
                }
            } else {
                dp[row][col] = (dp[row - 1][col + 1] + dp[row][col + 1] + dp[row + 1][col + 1])
                    % 1_000_000_007;
            }
        }
    }

    if v[0] != 0 {
        return dp[v[0]][0];
    }

    dp.iter()
        .map(|v| v[0])
        .fold(0, |acc, x| (acc + x) % 1_000_000_007)
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    println!(
        "{}",
        f(
            s.lines()
                .nth(0)
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse()
                .unwrap(),
            s.lines()
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        )
    );
}

#[cfg(test)]
mod test_array_description {
    use super::f;

    #[test]
    fn example() {
        assert_eq!(3, f(5, vec![2, 0, 2]));
    }
}
