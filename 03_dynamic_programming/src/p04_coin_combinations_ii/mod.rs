use std::io::Read;

fn f(sum_needed: usize, mut coins_available: Vec<usize>) -> usize {
    coins_available = coins_available
        .into_iter()
        .filter(|&c| c <= sum_needed)
        .collect();
    coins_available.sort_unstable();

    let mut ways = vec![0; sum_needed + 1];
    ways[0] = 1;

    for c in coins_available {
        for i in c..=sum_needed {
            ways[i] = (ways[i] + ways[i - c]) % 1_000_000_007;
        }
    }

    ways[sum_needed]
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    let sum_needed = s
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let coins_available = s
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", f(sum_needed, coins_available));
}

#[cfg(test)]
mod test_coin_combinations_ii {
    use super::f;

    #[test]
    fn example() {
        assert_eq!(3, f(9, vec![2, 3, 5]));
    }
}
