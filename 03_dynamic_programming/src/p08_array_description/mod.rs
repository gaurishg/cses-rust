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
    let mut dp = HashMap::new();
    if v[0] == 0 {
        return (1..=upper_bound)
            .map(|x| calculate(upper_bound, &v, 0, x, &mut dp))
            .fold(0, |acc, v| (acc + v) % 1_000_000_007);
    }

    let ret = calculate(upper_bound, &v, 0, v[0], &mut dp);
    eprintln!("{dp:?}");
    ret
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
