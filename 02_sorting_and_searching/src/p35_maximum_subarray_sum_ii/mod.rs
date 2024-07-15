use std::{collections::BTreeMap, i64, io::Read};

fn f(a: i64, b: i64, v: Vec<i64>) -> i64 {
    let mut m: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
    m.insert(0, vec![-1]);
    let mut running_sum = 0;

    for (idx, &x) in v.iter().enumerate() {
        running_sum += x;
        m.entry(running_sum).or_default().push(idx as i64);
    }

    let mut all_sums = m.keys().copied().collect::<Vec<i64>>();
    all_sums.sort();

    let mut result = None;
    for left_sum in all_sums.iter().copied() {
        for right_sum in all_sums.iter().copied().rev() {
            let maxsum = right_sum - left_sum;
            if result.is_none() || result.is_some() && result.unwrap() < maxsum {
                for &left_idx in m.get(&left_sum).unwrap() {
                    for _ in m
                        .get(&right_sum)
                        .unwrap()
                        .iter()
                        .rev()
                        .skip_while(|&&i| i as i64 > b + left_idx as i64)
                        .take_while(|&&i| i as i64 >= a + left_idx as i64)
                    {
                        result = Some(maxsum);
                        break;
                    }
                }
            }
        }
    }

    result.unwrap()
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);
    let a = s
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let b = s
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .nth(2)
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

    println!("{}", f(a, b, v));
}

#[cfg(test)]
mod test_maximum_subarray_sum_ii {
    use super::f;

    #[test]
    fn test_example() {
        assert_eq!(8, f(1, 2, vec![-1, 3, -2, 5, 3, -5, 2, 2]));
    }

    #[test]
    fn test_example2() {
        assert_eq!(1, f(1, 1, vec![1]));
    }

    #[test]
    fn test_1() {
        assert_eq!(390, f(7, 9, vec![58, 36, 81, -7, 46, 49, 87, -58, 98, -15]));
    }

    #[test]
    fn test_2() {
        assert_eq!(163, f(7, 7, vec![-22, 0, 78, -48, 94, 68, -7, -73, 8, 62]));
    }

    #[test]
    fn test_3() {
        assert_eq!(309, f(6, 10, vec![29, 78, 71, -11, 3, 3, 88, -21, 69, -96]));
    }

    #[test]
    fn test_14() {
        assert_eq!(-21919, f(1, 1, vec![-21919]));
    }
}
