use std::io::Read;

fn f(grid: Vec<Vec<char>>) -> usize {
    if grid[0][0] == '*' || grid[grid.len() - 1][grid.len() - 1] == '*' {
        return 0;
    }
    let mut dp = vec![vec![0; grid.len() + 1]; grid.len() + 1];
    dp[grid.len() - 1][grid.len()] = 1;

    for r in (0..grid.len()).rev() {
        for c in (0..grid.len()).rev() {
            if grid[r][c] != '*' {
                dp[r][c] = (dp[r + 1][c] + dp[r][c + 1]) % 1_000_000_007;
            }
        }
    }

    dp[0][0]
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);
    println!(
        "{}",
        f(s.lines().skip(1).map(|s| s.chars().collect()).collect())
    );
}

#[cfg(test)]
mod test_grid_paths {
    use super::f;

    #[test]
    fn example() {
        let s = String::from(
            "....
.*..
...*
*...",
        );
        assert_eq!(3, f(s.lines().map(|s| s.chars().collect()).collect()));
    }
}
