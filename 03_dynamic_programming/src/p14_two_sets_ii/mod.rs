use std::io::Read;

fn f(n: usize) -> usize {
    if (n * (n + 1)) % 4 != 0 {
        return 0;
    }

    let half_sum = (n * (n + 1)) / 4;

    let mut dp = vec![0; half_sum + 1];
    // f(start, sum_to_make) = \sum_{x=start}^n{f(x + 1, sum_to_make - x)}
    dp[0] = 1;

    // Find out why this does not work, which looks correct
    // This does not work because let's say there are 1_000_000_008 solutions
    // then after mod it becomes 1, and in the end we half it, so basically mod messes up
    // everything
    //for start in (1..=n).rev() {
    //    for to_make in start..=half_sum {
    //        // for every number check number of solutions if we take it plus
    //        // number of solutions if we leave it
    //        dp[start][to_make] =
    //            (dp[start + 1][to_make] + dp[start + 1][to_make - start]) % 1_000_000_007;
    //    }
    //}
    // Since every set is counted but we only need pairs, so half it
    //dp[1][half_sum] / 2

    // Why this works even though looks incorrect at first sight, because this solutions only takes number
    // upto n-1 into account, it assumes that nth number is already in the other set, so make our
    // solution without that. Using this method we do not need to halve the solution in the end
    // It is not necessary to go upto n-1, we can also start from 2 and go upto n, similarly we can
    // leave out any one number and our answer will be correct
    for start in (1..n).rev() {
        for to_make in (start..=half_sum).rev() {
            dp[to_make] = (dp[to_make] + dp[to_make - start]) % 1_000_000_007;
        }
    }

    dp[half_sum]
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);
    println!("{}", f(s.trim().parse().unwrap()));
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
