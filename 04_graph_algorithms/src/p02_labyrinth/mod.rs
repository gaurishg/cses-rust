use std::{collections::VecDeque, io::Read};

fn f(mut v: Vec<Vec<char>>) -> Option<String> {
    let n_rows = v.len();
    let n_cols = v[0].len();

    let mut a_pos = (0, 0);
    let mut b_pos = (0, 0);
    for r in 0..n_rows {
        for c in 0..n_cols {
            if v[r][c] == 'A' {
                a_pos = (r, c);
            } else if v[r][c] == 'B' {
                b_pos = (r, c);
            }
        }
    }

    let mut q = VecDeque::new();
    q.push_back(a_pos);
    v[b_pos.0][b_pos.1] = '.';

    while !q.is_empty() {
        let (r, c) = q.pop_front().unwrap();
        if (r, c) == b_pos {
            let (mut r, mut c) = b_pos;
            let mut s = String::new();
            while (r, c) != a_pos {
                s.push(v[r][c]);
                if v[r][c] == 'U' {
                    r += 1;
                } else if v[r][c] == 'D' {
                    r -= 1;
                } else if v[r][c] == 'L' {
                    c += 1;
                } else {
                    c -= 1;
                }
            }
            s = s.chars().rev().collect();
            return Some(s);
        }
        if r > 0 && v[r - 1][c] == '.' {
            q.push_back((r - 1, c));
            v[r - 1][c] = 'U';
        }
        if r < n_rows - 1 && v[r + 1][c] == '.' {
            q.push_back((r + 1, c));
            v[r + 1][c] = 'D';
        }
        if c > 0 && v[r][c - 1] == '.' {
            q.push_back((r, c - 1));
            v[r][c - 1] = 'L';
        }
        if c + 1 < n_cols && v[r][c + 1] == '.' {
            q.push_back((r, c + 1));
            v[r][c + 1] = 'R';
        }
    }

    None
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    if let Some(s) = f(s.lines().skip(1).map(|s| s.chars().collect()).collect()) {
        println!("YES\n{}\n{}", s.len(), s);
    } else {
        println!("NO");
    }
}

#[cfg(test)]
mod test_labyrinth {
    use super::f;

    fn s_to_vec(s: &str) -> Vec<Vec<char>> {
        s.lines().skip(1).map(|s| s.chars().collect()).collect()
    }

    #[test]
    fn example() {
        assert_eq!(
            f(s_to_vec(
                "########
#.A#...#
#.##.#B#
#......#
########"
            )),
            Some("LDDRRRRRU".to_owned())
        );
    }
}
