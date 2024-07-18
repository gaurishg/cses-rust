use std::io::Read;

fn f(coins: Vec<usize>) -> Vec<usize> {
    let mut result = vec![];

    for coin in coins {
        let n = result.len();
        result.push(coin);
        for i in 0..n {
            result.push(coin + result[i]);
        }

        result.sort_unstable();
        result.dedup();
    }

    result
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
