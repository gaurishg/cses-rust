use std::{collections::HashMap, io::Read};

fn f(target_sum: i64, v: Vec<i64>) -> i64 {
    let mut m = HashMap::from([(0, 1)]);
    let mut counts = 0;
    let mut s = 0;

    for x in v {
        s += x;
        if let Some(cnt) = m.get(&(s - target_sum)) {
            counts += cnt;
        }
        *m.entry(s).or_default() += 1;
    }

    counts
}

pub fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_to_string(&mut s);
    let target_sum = s
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let v = s
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{}", f(target_sum, v));
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_example() {
        assert_eq!(2, f(7, vec![2, -1, 3, 5, -2]));
    }

    #[test]
    fn test_1() {
        assert_eq!(
            51,
            f(
                50,
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ]
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            5050,
            f(
                0,
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ]
            )
        );
    }
}
