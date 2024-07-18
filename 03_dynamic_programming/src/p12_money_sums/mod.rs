use std::io::Read;

fn f(coins: Vec<usize>) -> Vec<usize> {
    let mut sum_is_possible = vec![false; 1 + coins.iter().sum::<usize>()];
    sum_is_possible[0] = true;

    let mut total_upto_here = 0;
    for coin in coins {
        total_upto_here += coin;
        for amt in (coin..=total_upto_here).rev() {
            sum_is_possible[amt] = sum_is_possible[amt] || sum_is_possible[amt - coin];
        }
    }

    sum_is_possible
        .into_iter()
        .enumerate()
        .skip(1)
        .filter(|&(_, x)| x)
        .map(|(idx, _)| idx)
        .collect()
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    let result = f(s
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect());

    println!("{}", result.len());

    for x in result {
        print!("{x} ");
    }
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn example() {
        assert_eq!(vec![2, 4, 5, 6, 7, 8, 9, 11, 13], f(vec![4, 2, 5, 2]));
    }
}
