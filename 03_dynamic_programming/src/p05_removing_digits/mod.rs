use std::io::Read;

fn f(n: usize) -> usize {
    let mut steps = [n + 1; 11];
    steps[10] = 0;

    for mut x in 1..=n {
        steps.copy_within(1..11, 0);
        steps[10] = n + 1;
        let x_again = x;
        while x > 0 {
            let digit = x % 10;
            x /= 10;
            if digit <= x_again {
                steps[10] = std::cmp::min(steps[10], 1 + steps[10 - digit]);
            }
        }
    }

    steps[10]
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
