use std::{collections::HashMap, io::Read};

fn f(v: Vec<i64>) -> i64 {
    let n = v.len() as i64;
    let mut m = HashMap::from([(0, 1)]);
    let mut sum = 0;
    let mut counts = 0;

    for x in v {
        sum += x;
        sum = sum.rem_euclid(n);
        if let Some(cnt) = m.get(&sum) {
            counts += cnt;
        }
        *m.entry(sum).or_default() += 1;
    }

    counts
}

pub fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_to_string(&mut s);
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
mod test {
    use super::f;

    #[test]
    fn test_example() {
        assert_eq!(1, f(vec![3, 1, 2, 7, 4]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            47,
            f(vec![
                2, 1, -3, 2, -7, 7, -2, 6, 9, -4, 10, -6, 3, 9, -8, 7, -2, -9, 4, -3, -2, 6, 6, 3,
                7, 2, -1, 10, 6, -4, 4, 9, -1, -5, -6, -9, 1, 2, 2, -10, -2, 3, 3, 4, 3, -6, -5,
                -1, 9, 6, -4, 6, 2, -1, 6, 1, 6, 1, 3, 7, -6, 10, 1, 1, 6, -9, 0, 5, -1, 8, 6, 0,
                5, 5, -3, 1, 1, -5, -9, -8, -9, -7, 7, -6, 10, 7, 8, 1, -2, 2, 8, 9, -1, 5, -7, 3,
                -3, -9, -3, 4
            ])
        );
    }
}
