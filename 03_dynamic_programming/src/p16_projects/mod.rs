use std::{collections::HashMap, io::Read};

// max_profit
//      = f(index=i, end_time=t)
//      = MAX[v[i].2 + f(index=i+1, end_time=v[i].1),
//          f(index=i+1, end_time=t)]
fn f(mut v: Vec<(usize, usize, usize)>) -> usize {
    v.sort_unstable_by_key(|ele| ele.1);

    let mut times = v.iter().flat_map(|&x| [x.1, x.0]).collect::<Vec<usize>>();
    times.push(0);
    times.sort_unstable();
    times.dedup();
    let times = times;
    let times_mapping: HashMap<usize, usize> =
        times.iter().enumerate().map(|(idx, x)| (*x, idx)).collect();

    let mut dp = vec![0; times.len()];
    for (start_time, end_time, profit) in v.into_iter().rev() {
        let start_time = times_mapping[&start_time];
        let end_time = times_mapping[&end_time];
        for last_time in (0..dp.len()).rev() {
            if start_time > last_time {
                dp[last_time] = std::cmp::max(profit + dp[end_time], dp[last_time]);
            }
        }
    }

    dp[0]
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    println!(
        "{}",
        f(s.lines()
            .skip(1)
            .map(|s| s.split_whitespace().map(|x| x.parse().unwrap()))
            .map(|mut it| {
                let ele0 = it.next().unwrap();
                let ele1 = it.next().unwrap();
                let ele2 = it.next().unwrap();
                (ele0, ele1, ele2)
            })
            .collect())
    );
}

#[cfg(test)]
mod test_projects {
    use super::f;

    #[test]
    fn example() {
        assert_eq!(f(vec![(2, 4, 4), (3, 6, 6), (6, 8, 2), (5, 7, 3)]), 7);
    }

    #[test]
    fn test_1() {
        let v = vec![
            (14, 14, 98),
            (76, 76, 58),
            (94, 94, 57),
            (92, 92, 45),
            (82, 82, 14),
            (86, 86, 41),
            (87, 87, 72),
            (14, 14, 26),
            (27, 27, 85),
            (48, 48, 52),
        ];
        assert_eq!(f(v), 522);
    }

    #[test]
    fn test_2() {
        let v = vec![
            (76, 77, 96),
            (77, 78, 11),
            (11, 12, 46),
            (43, 44, 82),
            (25, 25, 87),
            (96, 97, 4),
            (39, 40, 22),
            (42, 42, 62),
            (42, 42, 30),
            (88, 88, 19),
        ];
        assert_eq!(f(v), 418);
    }

    #[test]
    fn test_3() {
        let v = vec![
            (58, 59, 91),
            (8, 10, 24),
            (32, 35, 50),
            (24, 24, 21),
            (91, 93, 9),
            (90, 92, 97),
            (55, 57, 50),
            (57, 59, 54),
            (37, 39, 60),
            (41, 41, 26),
        ];
        assert_eq!(f(v), 419);
    }

    #[test]
    fn test_4() {
        let v = vec![
            (39, 45, 23),
            (65, 65, 37),
            (54, 61, 45),
            (98, 100, 20),
            (12, 15, 90),
            (22, 31, 88),
            (50, 52, 36),
            (68, 74, 97),
            (24, 32, 36),
            (97, 99, 11),
        ];
        assert_eq!(f(v), 436);
    }

    #[test]
    fn test_5() {
        let v = vec![
            (86, 94, 24),
            (88, 94, 35),
            (50, 86, 23),
            (15, 29, 53),
            (66, 72, 82),
            (61, 84, 93),
            (16, 40, 22),
            (92, 99, 70),
            (41, 93, 24),
            (78, 86, 19),
        ];
        assert_eq!(f(v), 224);
    }
}
