use std::io::Read;

fn is_possible(k: usize, v: &Vec<usize>, max_possible_sum: usize) -> bool {
    v.iter()
        .fold(
            (0usize, 1usize),
            |(running_sum, number_of_partitions), &x| {
                if running_sum + x > max_possible_sum {
                    (x, number_of_partitions + 1)
                } else {
                    (running_sum + x, number_of_partitions)
                }
            },
        )
        .1
        <= k
}

fn f(k: usize, v: Vec<usize>) -> usize {
    let mut l = *v.iter().max().unwrap();
    let mut r = v.iter().sum();

    while r - l >= 3 {
        let mid = (l + r) / 2;
        if is_possible(k, &v, mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    for min_sum in l..=r {
        if is_possible(k, &v, min_sum) {
            return min_sum;
        }
    }

    0
}

pub fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_to_string(&mut s);

    let k = s
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
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", f(k, v));
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_example() {
        assert_eq!(8, f(3, vec![2, 4, 7, 3, 5]));
    }

    #[test]
    fn test_7() {
        assert_eq!(
            1000,
            f(
                100,
                vec![
                    460, 464, 207, 712, 844, 329, 168, 357, 620, 398, 158, 30, 576, 443, 497, 506,
                    634, 863, 272, 465, 278, 706, 845, 900, 932, 605, 516, 713, 482, 12, 37, 418,
                    485, 620, 1000, 284, 929, 405, 180, 523, 122, 660, 91, 623, 371, 639, 116, 391,
                    965, 121, 162, 549, 78, 38, 936, 383, 374, 946, 937, 50, 685, 281, 200, 959,
                    492, 615, 185, 457, 931, 339, 992, 161, 928, 972, 104, 112, 657, 754, 419, 529,
                    862, 104, 416, 966, 966, 813, 9, 715, 897, 112, 102, 891, 318, 376, 542, 599,
                    144, 834, 78, 152
                ]
            )
        );
    }
}
