use std::io::Read;

const MOD: usize = 1_000_000_000 + 7;

fn f(n: usize) -> usize {
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        for x in 1..=std::cmp::min(i, 6) {
            dp[i] = (dp[i] + dp[i - x]) % MOD;
        }
    }

    dp[n]
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);
    println!("{}", f(s.trim().parse().unwrap()));
}

#[cfg(test)]
mod test_dice_combinations {
    use super::f;

    #[test]
    fn test_example() {
        assert_eq!(4, f(3));
    }
}
