use std::io::Read;

fn f(target_sum: i64, v: Vec<i64>) -> (usize, usize, usize, usize) {
    let mut v: Vec<(i64, usize)> = v.into_iter().enumerate().map(|(idx, x)| (x, idx)).collect();
    v.sort_unstable();
    let v = v;

    for (idx, &(x, ix)) in v.iter().take_while(|(x, _)| *x <= target_sum).enumerate() {
        for (idy, &(y, iy)) in v
            .iter()
            .take_while(|&(y, _)| x + y <= target_sum)
            .enumerate()
            .skip(idx + 1)
        {
            for (idz, &(z, iz)) in v
                .iter()
                .take_while(|&(z, _)| x + y + z <= target_sum)
                .enumerate()
                .skip(idy + 1)
            {
                if let Ok(idw) = v[idz + 1..]
                    .binary_search_by_key(&(target_sum - x - y - z), |(value, _)| *value)
                {
                    let mut v = vec![ix + 1, iy + 1, iz + 1, v[idw + idz + 1].1 + 1];
                    v.sort_unstable();
                    return (v[0], v[1], v[2], v[3]);
                }
            }
        }
    }
    (0, 0, 0, 0)
}

pub fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);
    let target_sum = buf
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let v = buf
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    match f(target_sum, v) {
        (0, 0, 0, 0) => println!("IMPOSSIBLE"),
        (x, y, z, w) => println!("{x} {y} {z} {w}"),
    }
}

#[cfg(test)]
mod test {
    use super::f;

    fn check_result(target_sum: i64, v: Vec<i64>) -> bool {
        let (ix, iy, iz, iw) = {
            let (ix, iy, iz, iw) = f(target_sum, v.clone());
            (ix - 1, iy - 1, iz - 1, iw - 1)
        };

        target_sum == v[ix] + v[iy] + v[iz] + v[iw]
    }

    #[test]
    fn test_1() {
        assert!(check_result(15, vec![3, 2, 5, 8, 1, 3, 2, 3]));
    }
}
