use std::{io::Read, usize};

#[allow(non_snake_case)]
fn recurse(v: &mut Vec<Vec<char>>, r: isize, c: isize, R: isize, C: isize) {
    if r < 0 || r >= R || c < 0 || c >= C || v[r as usize][c as usize] == '#' {
        return;
    }

    v[r as usize][c as usize] = '#';
    recurse(v, r - 1, c, R, C);
    recurse(v, r + 1, c, R, C);
    recurse(v, r, c - 1, R, C);
    recurse(v, r, c + 1, R, C);
}

#[allow(non_snake_case)]
fn f(mut v: Vec<Vec<char>>) -> usize {
    let R = v.len() as isize;
    let C = v[0].len() as isize;
    let mut rooms = 0;

    for r in 0..v.len() {
        for c in 0..v[0].len() {
            if v[r][c] != '#' {
                rooms += 1;
            }
            recurse(&mut v, r as isize, c as isize, R, C);
        }
    }

    rooms
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
mod test_counting_rooms {
    use super::f;

    fn s_to_vec(s: &str) -> Vec<Vec<char>> {
        s.lines().map(|s| s.chars().collect()).collect()
    }

    #[test]
    fn example() {
        let s = "########
#..#...#
####.#.#
#..#...#
########";
        assert_eq!(f(s_to_vec(s)), 3);
    }
}
