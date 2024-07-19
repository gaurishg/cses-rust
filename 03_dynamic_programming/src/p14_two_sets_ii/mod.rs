fn f(n: usize) -> usize {
    if (n * (n + 1)) % 4 != 0 {
        return 0;
    }

    let half_sum = (n * (n + 1)) / 4;

    let mut dp = vec![vec![0; half_sum + 1]; n + 2];
    // f(start, sum_to_make) = \sum_{x=start}^n{f(x + 1, sum_to_make - x)}
    for v in &mut dp {
        v[0] = 1;
    }

    for start in (1..=n).rev() {
        for to_make in start..=half_sum {
            //for x in start..=std::cmp::min(to_make, n) {
            //    dp[start][to_make] = (dp[start][to_make] + dp[x + 1][to_make - x]) % 1_000_000_007;
            //}
            //dp[start][to_make] = (start..=std::cmp::min(to_make, n))
            //    .map(|x| dp[x + 1][to_make - x])
            //    .fold(0, |acc, x| (acc + x) % 1_000_000_007);
            dp[start][to_make] =
                (dp[start + 1][to_make] + dp[start + 1][to_make - start]) % 1_000_000_007;
        }
    }

    dp[1][half_sum] / 2
}

#[cfg(test)]
mod test_two_sets_ii {
    use super::f;

    #[test]
    fn example() {
        assert_eq!(f(7), 4);
    }

    #[test]
    fn test_1() {
        assert_eq!(f(1), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(f(2), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(f(3), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(f(4), 1);
    }

    #[test]
    fn test_5() {
        assert_eq!(f(5), 0);
    }

    #[test]
    fn test_6() {
        assert_eq!(f(6), 0);
    }

    #[test]
    fn test_7() {
        assert_eq!(f(7), 4);
    }

    #[test]
    fn test_8() {
        assert_eq!(f(8), 7);
    }

    #[test]
    fn test_9() {
        assert_eq!(f(9), 0);
    }

    #[test]
    fn test_10() {
        assert_eq!(f(10), 0);
    }

    #[test]
    fn test_11() {
        assert_eq!(f(107), 5899_38_798);
    }

    #[test]
    fn test_12() {
        assert_eq!(f(112), 9791_44_036);
    }

    #[test]
    fn test_13() {
        assert_eq!(f(114), 0);
    }

    #[test]
    fn test_15() {
        assert_eq!(f(147), 3488_26_222);
    }
}
