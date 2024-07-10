use std::io::Read;

fn f(target_sum: i64, v: Vec<i64>) -> (usize, usize, usize) {
    let mut v: Vec<(i64, usize)> = v.into_iter().enumerate().map(|(idx, x)| (x, idx)).collect();
    v.sort_unstable();

    for (idx, (x, _)) in v.iter().take_while(|(x, _)| *x <= target_sum).enumerate() {
        for (idy, (y, _)) in v
            .iter()
            .take_while(|(y, _)| *x + *y <= target_sum)
            .enumerate()
            .skip(idx + 1)
        {
            if let Ok(ret) =
                v[idy + 1..].binary_search_by_key(&(target_sum - *x - *y), |&(value, _)| value)
            {
                let idz = ret + idy + 1;
                let mut v = vec![v[idx].1 + 1, v[idy].1 + 1, v[idz].1 + 1];
                v.sort_unstable();
                return (v[0], v[1], v[2]);
            }
        }
    }

    (0, 0, 0)
}

pub fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);
    let mut lines = buf.lines();
    let target_sum = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let v = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    match f(target_sum, v) {
        (0, 0, 0) => println!("IMPOSSIBLE"),
        (x, y, z) => println!("{x} {y} {z}"),
    }
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_1() {
        assert_eq!((1, 3, 4), f(8, vec![2, 7, 5, 1]));
    }
}
