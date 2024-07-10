use std::io::Read;

fn f(target_sum: i64, v: Vec<i64>) -> i64 {
    let mut l = 0;
    let mut r = 0;
    let mut sum = 0;
    let mut counts = 0;

    while r < v.len() {
        sum += v[r];
        if sum == target_sum {
            counts += 1;
        }
        while l <= r && sum >= target_sum {
            sum -= v[l];
            l += 1;
            if sum == target_sum {
                counts += 1;
            }
        }
        r += 1;
    }
    counts
}

pub fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_to_string(&mut s);
    let target_sum = s
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
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", f(target_sum, v));
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_1() {
        assert_eq!(3, f(7, vec![2, 4, 1, 2, 7]));
    }
}
