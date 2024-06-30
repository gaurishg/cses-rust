fn f(wts: Vec<i64>) -> i64 {
    let total: i64 = wts.iter().sum();
    (0i64..(1 << wts.len()))
        .map(|x| {
            let mut wt = 0;
            for (idx, c) in format!("{:b}", x).chars().rev().enumerate() {
                if c == '1' {
                    wt += wts[idx];
                }
            }
            (total - 2 * wt).abs()
        })
        .min()
        .unwrap()
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    buffer.clear();
    let _ = std::io::stdin().read_line(&mut buffer);
    let wts = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", f(wts));
}

#[cfg(test)]
mod my_tests {
    use super::f;
    #[test]
    fn test_1() {
        assert_eq!(f(vec![3, 2, 7, 4, 1]), 1);
    }

    #[test]
    fn test_single_item() {
        assert_eq!(f(vec![100]), 100);
    }

    #[test]
    fn test_2() {
        let v = "934033764 747013925 113297529 621350044 4858224 896418401 823418019 490285020 811592918 318107753 122431099 971962557 68572395 269437889 506050802 903504371 908615271 406858603 39392057 816479349".split_ascii_whitespace().map(|x|x.parse().unwrap()).collect();
        assert_eq!(f(v), 5482);
    }
}
