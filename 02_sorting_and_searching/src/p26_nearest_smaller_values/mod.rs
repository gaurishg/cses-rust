use std::io::Read;

fn f(v: Vec<usize>) -> Vec<usize> {
    let mut monotonic_increasing_stack = vec![0];
    let mut result = vec![0];

    for (idx, &x) in v.iter().enumerate().skip(1) {
        while !monotonic_increasing_stack.is_empty()
            && v[*monotonic_increasing_stack.last().unwrap()] >= x
        {
            monotonic_increasing_stack.pop();
        }

        if monotonic_increasing_stack.is_empty() {
            result.push(0);
        } else {
            result.push(monotonic_increasing_stack.last().unwrap() + 1);
        }

        monotonic_increasing_stack.push(idx);
    }

    result
}

pub fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);

    for x in f(buf
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect())
    {
        print!("{x} ");
    }
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![0, 1, 0, 3, 4, 3, 3, 7],
            f(vec![2, 5, 1, 4, 8, 3, 2, 5])
        );
    }
}
