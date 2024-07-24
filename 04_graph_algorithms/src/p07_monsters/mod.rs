use std::{collections::VecDeque, io::Read};

fn f(mut maze: Vec<Vec<char>>) -> Option<String> {
    let n_rows = maze.len();
    let n_cols = maze[0].len();
    let mut monster_q = VecDeque::new();
    let mut q = VecDeque::new();
    let mut a_pos = (0, 0);
    let mut result = String::new();

    for r in 0..n_rows {
        for c in 0..n_cols {
            if maze[r][c] == 'M' {
                monster_q.push_back((r, c));
            } else if maze[r][c] == 'A' {
                q.push_back((r, c));
                a_pos = (r, c);
            }
        }
    }

    let mut parents: Vec<Vec<_>> = (0..n_rows)
        .map(|r| (0..n_cols).map(|c| (r, c)).collect())
        .collect();

    while !q.is_empty() {
        let n = monster_q.len();
        for _ in 0..n {
            let (r, c) = monster_q.pop_front().unwrap();
            if r > 0 && maze[r - 1][c] != '#' && maze[r - 1][c] != 'M' {
                monster_q.push_back((r - 1, c));
                maze[r - 1][c] = 'M';
            }
            if r < n_rows - 1 && maze[r + 1][c] != '#' && maze[r + 1][c] != 'M' {
                monster_q.push_back((r + 1, c));
                maze[r + 1][c] = 'M';
            }
            if c > 0 && maze[r][c - 1] != '#' && maze[r][c - 1] != 'M' {
                monster_q.push_back((r, c - 1));
                maze[r][c - 1] = 'M';
            }
            if c < n_cols - 1 && maze[r][c + 1] != '#' && maze[r][c + 1] != 'M' {
                monster_q.push_back((r, c + 1));
                maze[r][c + 1] = 'M';
            }
        }

        let n = q.len();
        for _ in 0..n {
            let (r, c) = q.pop_front().unwrap();
            if r == 0 || r == n_rows - 1 || c == 0 || c == n_cols - 1 {
                let (mut current_r, mut current_c) = (r, c);
                while (current_r, current_c) != a_pos {
                    let (pr, pc) = parents[current_r][current_c];
                    if pr == current_r - 1 {
                        result.push('D');
                    } else if pr == current_r + 1 {
                        result.push('U');
                    } else if pc == current_c - 1 {
                        result.push('R');
                    } else {
                        result.push('L');
                    }
                    (current_r, current_c) = (pr, pc);
                }
                return Some(result.chars().rev().collect());
            }
            if r > 0 && maze[r - 1][c] != '#' && maze[r - 1][c] != 'M' && maze[r - 1][c] != 'A' {
                q.push_back((r - 1, c));
                parents[r - 1][c] = (r, c);
                maze[r - 1][c] = 'A';
            }
            if r < n_rows - 1
                && maze[r + 1][c] != '#'
                && maze[r + 1][c] != 'M'
                && maze[r + 1][c] != 'A'
            {
                q.push_back((r + 1, c));
                parents[r + 1][c] = (r, c);
                maze[r + 1][c] = 'A';
            }
            if c > 0 && maze[r][c - 1] != '#' && maze[r][c - 1] != 'M' && maze[r][c - 1] != 'A' {
                q.push_back((r, c - 1));
                parents[r][c - 1] = (r, c);
                maze[r][c - 1] = 'A';
            }
            if c < n_cols - 1
                && maze[r][c + 1] != '#'
                && maze[r][c + 1] != 'M'
                && maze[r][c + 1] != 'A'
            {
                q.push_back((r, c + 1));
                parents[r][c + 1] = (r, c);
                maze[r][c + 1] = 'A';
            }
        }
    }
    None
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    if let Some(s) = f(s.lines().skip(1).map(|l| l.chars().collect()).collect()) {
        println!("YES\n{}\n{}", s.len(), s);
    } else {
        println!("NO");
    }
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn example() {
        let s = "########
#M..A..#
#.#.M#.#
#M#..#..
#.######"
            .lines()
            .map(|s| s.chars().collect())
            .collect();
        assert_eq!(f(s), Some("RRDDR".into()));
    }
}
