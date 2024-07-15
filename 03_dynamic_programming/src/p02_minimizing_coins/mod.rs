// f(sum) = 1 + min(f(sum-x)) where x is various coin denominations

use std::io::Read;

fn f(sum: isize, mut coins: Vec<usize>) -> isize {
    coins.sort_unstable();

    let mut coins_needed = vec![2 * sum; sum as usize + 1];
    coins_needed[0] = 0;

    for i in 1..coins_needed.len() {
        for c in coins.iter().take_while(|&&c| c <= i) {
            coins_needed[i] = std::cmp::min(coins_needed[i], 1 + coins_needed[i - c]);
        }
    }

    if *coins_needed.last().unwrap() >= 2 * sum {
        -1
    } else {
        *coins_needed.last().unwrap()
    }
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    let sum = s
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let coins = s
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", f(sum, coins));
}

#[cfg(test)]
mod test_minimizing_coins {
    use super::f;

    #[test]
    fn test_example() {
        assert_eq!(3, f(11, vec![1, 5, 7]));
    }

    #[test]
    fn test_example2() {
        assert_eq!(-1, f(1, vec![2]));
    }

    #[test]
    fn test_example3() {
        assert_eq!(-1, f(4, vec![3, 5]));
    }
}
