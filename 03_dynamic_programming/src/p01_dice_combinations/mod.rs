use std::io::Read;

const MOD: usize = 1_000_000_000 + 7;

fn f(n: usize) -> usize {
    let mut dp = [0; 7];
    dp[6] = 1;

    for _ in 0..n {
        dp.copy_within(1..7, 0);
        dp[6] = dp[0..6].iter().sum::<usize>() % MOD;
    }

    dp[6]
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
