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
            let mut idz = idy + 1;
            let mut idw = v.len() - 1;
            while idz < idw {
                if v[idz].0 + v[idw].0 + x + y == target_sum {
                    let mut v = vec![ix + 1, iy + 1, v[idz].1 + 1, v[idw].1 + 1];
                    v.sort_unstable();
                    return (v[0], v[1], v[2], v[3]);
                } else if v[idz].0 + v[idw].0 + x + y < target_sum {
                    idz += 1;
                } else {
                    idw -= 1;
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
