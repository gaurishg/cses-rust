use std::{collections::HashMap, io::Read, usize};
//const N: usize = 1_000_001;
const N: usize = 10;

fn calculate(a: usize, b: usize, dp: &mut Vec<HashMap<usize, usize>>) -> usize {
    if dp[a].get(&b).is_some() {
        return dp[a][&b];
    }

    let mut total = 0;

    for piece_a in 0..=a {
        for piece_b in 0..=b {
            if piece_a == 0 && piece_b == 0 {
                continue;
            }
            total = (total + calculate(a - piece_a, b - piece_b, dp)) % 1_000_000_007;
        }
    }

    if a == b {
        for x in 1..=a {
            total = (total + calculate(a - x, b - x, dp)) % 1_000_000_007;
        }
    }

    dp[a].insert(b, total);
    total
}

fn f(v: Vec<usize>) -> Vec<usize> {
    let mut dp: Vec<HashMap<usize, usize>> = vec![HashMap::new(); N];
    dp[0].insert(0, 1);
    //dp[1].insert(1, 2);

    let ret = v.into_iter().map(|x| calculate(x, x, &mut dp)).collect();
    dbg!(dp);
    ret
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    // TODO: solve it
    for x in f(s.lines().skip(1).map(|s| s.parse().unwrap()).collect()) {
        println!("{x}");
    }
}

#[cfg(test)]
mod test_counting_towers {
    use super::f;

    #[test]
    #[ignore]
    fn example_1() {
        assert_eq!(vec![2], f(vec![1]));
    }

    #[test]
    #[ignore]
    fn example_2() {
        assert_eq!(vec![8], f(vec![2]));
    }

    #[test]
    #[ignore]
    fn example1() {
        assert_eq!(vec![8, 2864, 64_040_3945], f(vec![2, 6, 1337]));
    }
}
