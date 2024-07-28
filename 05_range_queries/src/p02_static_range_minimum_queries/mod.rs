use std::{io::Read, usize};

fn f(v: Vec<usize>, queries: Vec<(usize, usize)>) -> Vec<usize> {
    let log_limit = (v.len() as f64).log2() as usize;
    // dp[i][j] = min of starting at i and of length 1 << j
    let mut dp = vec![vec![usize::MAX; log_limit + 1]; v.len()];
    for (idx, &x) in v.iter().enumerate() {
        dp[idx][0] = x;
    }

    for j in 1..=log_limit {
        for i in (0..v.len()).take_while(|&i| (i + (1 << j)) <= v.len()) {
            dp[i][j] = dp[i][j - 1].min(dp[i + (1 << (j - 1))][j - 1]);
        }
    }

    let get_min = |a: usize, b: usize| {
        let a = a - 1;
        let b = b - 1;
        let len = b - a + 1;
        let mut start = a;
        let mut result = usize::MAX;
        for j in 0..20 {
            if len & (1 << j) != 0 {
                result = result.min(dp[start][j]);
                start += 1 << j;
            }
        }
        result
    };

    queries.into_iter().map(|(a, b)| get_min(a, b)).collect()
}

fn parse_input(s: &str) -> (Vec<usize>, Vec<(usize, usize)>) {
    (
        s.lines()
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(),
        s.lines()
            .skip(2)
            .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()))
            .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
            .collect(),
    )
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);
    let (v, q) = parse_input(&s);
    for x in f(v, q) {
        println!("{x}");
    }
}

#[cfg(test)]
mod test_static_range_minimum_queries {
    use super::{f, parse_input};

    #[test]
    fn example() {
        let s = "8 4
3 2 4 5 1 1 5 3
2 4
5 6
1 8
3 3";
        let (v, q) = parse_input(s);
        assert_eq!(f(v, q), vec![2, 1, 1, 4]);
    }
}
