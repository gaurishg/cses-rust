use std::{collections::HashMap, io::Read, isize};

fn count(
    board: &Vec<Vec<char>>,
    cols: &mut HashMap<isize, isize>,
    d1: &mut HashMap<isize, isize>,
    d2: &mut HashMap<isize, isize>,
    r: isize,
) -> usize {
    if r == 8 {
        return 1;
    }

    let mut total = 0;
    for (c, _) in board[r as usize]
        .iter()
        .enumerate()
        .filter(|(_, &ch)| ch != '*')
    {
        if *cols.entry(c as isize).or_default() == 0
            && *d1.entry(r + c as isize).or_default() == 0
            && *d2.entry(r - c as isize).or_default() == 0
        {
            cols.entry(c as isize).and_modify(|e| *e += 1).or_insert(1);
            *d1.entry(r + c as isize).or_default() += 1;
            *d2.entry(r - c as isize).or_default() += 1;
            total += count(board, cols, d1, d2, r + 1);
            cols.entry(c as isize).and_modify(|e| *e -= 1);
            d1.entry(r + c as isize).and_modify(|e| *e -= 1);
            d2.entry(r - c as isize).and_modify(|e| *e -= 1);
        }
    }

    total
}

fn f(board: &Vec<Vec<char>>) -> usize {
    let mut cols = HashMap::new();
    let mut d1 = HashMap::new();
    let mut d2 = HashMap::new();

    return count(board, &mut cols, &mut d1, &mut d2, 0);
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_to_string(&mut buffer);
    let board: Vec<Vec<char>> = buffer.lines().map(|s| s.chars().collect()).collect();
    println!("{}", f(&board));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        let string = String::from(
            "........
........
..*.....
........
........
.....**.
...*....
........",
        );
        let board = string.lines().map(|s| s.chars().collect()).collect();
        assert_eq!(65, super::f(&board));
    }
}
