fn f(n: i32) -> Vec<i32> {
    let mut v = vec![];
    let mut result = vec![];
    for x in 1..=n {
        v.push(x);
    }

    let mut start_from_start = false;
    while v.len() > 0 {
        if start_from_start {
            for &x in v.iter().step_by(2) {
                result.push(x);
            }
            if v.len() % 2 == 1 {
                start_from_start = false;
            }
            v = v.into_iter().skip(1).step_by(2).collect();
        } else {
            for &x in v.iter().skip(1).step_by(2) {
                result.push(x);
            }
            if v.len() % 2 == 1 {
                start_from_start = true;
            }
            v = v.into_iter().step_by(2).collect();
        }
    }

    result
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n = buffer.trim().parse().unwrap();

    for x in f(n) {
        print!("{x} ");
    }
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_1() {
        assert_eq!(vec![2, 4, 6, 1, 5, 3, 7], f(7));
    }
}
