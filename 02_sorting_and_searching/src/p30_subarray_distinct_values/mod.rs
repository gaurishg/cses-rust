use std::{collections::HashMap, io::Read, usize};

fn f(k: usize, v: Vec<usize>) -> usize {
    let mut map = HashMap::<usize, usize>::new();
    let mut l = 0;
    let mut r = 0;
    let mut count = 0;

    while r < v.len() {
        *map.entry(v[r]).or_default() += 1;
        r += 1;
        if map.len() <= k {
            count += r - l;
        } else {
            while map.len() > k {
                if *map.get(&v[l]).unwrap() == 1 {
                    map.remove(&v[l]);
                    l += 1;
                } else {
                    *map.get_mut(&v[l]).unwrap() -= 1;
                    l += 1;
                }
            }
            count += r - l;
        }
    }

    count
}

pub fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_to_string(&mut s);
    let k = s
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
    println!("{}", f(k, v));
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_single_element() {
        assert_eq!(1, f(2, vec![1]));
    }

    #[test]
    fn test_example() {
        assert_eq!(10, f(2, vec![1, 2, 3, 1, 1]));
    }

    #[test]
    fn test_1() {
        assert_eq!(
            641,
            f(
                3,
                vec![
                    3, 2, 3, 4, 3, 3, 4, 2, 3, 1, 4, 4, 1, 3, 4, 4, 3, 1, 3, 1, 4, 2, 2, 3, 4, 3,
                    2, 1, 1, 1, 4, 1, 1, 2, 2, 1, 3, 2, 4, 3, 1, 3, 4, 2, 1, 3, 2, 2, 2, 1, 4, 4,
                    1, 4, 3, 3, 3, 1, 2, 1, 2, 3, 1, 2, 4, 3, 1, 2, 4, 3, 1, 4, 3, 2, 1, 4, 3, 4,
                    1, 2, 3, 3, 2, 2, 2, 4, 4, 4, 3, 2, 2, 3, 4, 2, 4, 2, 4, 3, 1, 1
                ]
            )
        );
    }
}
