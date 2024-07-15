use std::io::Read;

fn f(n: usize) -> usize {
    let mut steps = vec![n + 1; n + 1];
    steps[0] = 0;

    for mut x in 1..=n {
        let x_again = x;
        while x > 0 {
            let digit = x % 10;
            x /= 10;
            if digit <= x_again {
                steps[x_again] = std::cmp::min(steps[x_again], 1 + steps[x_again - digit]);
            }
        }
    }

    steps[n]
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    println!("{}", f(s.trim().parse().unwrap()));
}

#[cfg(test)]
mod test_removing_digits {
    use super::f;

    #[test]
    fn example() {
        assert_eq!(5, f(27));
    }
}
