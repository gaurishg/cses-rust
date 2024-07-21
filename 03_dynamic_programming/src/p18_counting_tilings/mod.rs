use std::{collections::HashMap, io::Read, usize};

fn next_rows_recursive(
    current_row: usize,
    current_state: usize,
    next_row: usize,
    i: usize,
    c: usize,
    rows_map: &mut HashMap<usize, Vec<usize>>,
) {
    if i == c {
        if rows_map.contains_key(&current_row) {
            rows_map.get_mut(&current_row).unwrap().push(next_row);
        } else {
            rows_map.insert(current_row, vec![next_row]);
        }
        //rows_map
        //    .entry(current_row)
        //    .and_modify(|v| v.push(next_row))
        //    .or_insert(vec![next_row]);
        return;
    }

    if (current_state >> i) & 1 == 1 {
        next_rows_recursive(current_row, current_state, next_row, i + 1, c, rows_map);
        return;
    }

    let mut j = i + 1;
    while j < c && current_row & (1 << j) != 0 {
        j += 1;
    }

    if j == i + 1 && j < c {
        next_rows_recursive(
            current_row,
            current_state | (1 << i),
            next_row | (1 << i),
            i + 1,
            c,
            rows_map,
        );
        next_rows_recursive(
            current_row,
            current_state | (1 << i) | (1 << j),
            next_row,
            i + 2,
            c,
            rows_map,
        );
        return;
    }

    next_rows_recursive(
        current_row,
        current_state | (1 << i),
        next_row | (1 << i),
        i + 1,
        c,
        rows_map,
    );
}

fn next_row(
    current_row: usize,
    c: usize,
    rows_map: &mut HashMap<usize, Vec<usize>>,
) -> &Vec<usize> {
    if rows_map.contains_key(&current_row) {
        return &rows_map[&current_row];
    }

    next_rows_recursive(current_row, current_row, 0, 0, c, rows_map);

    &rows_map[&current_row]
}

//fn fill_a_row(
//    original_state: usize,
//    mut current_col: usize,
//    mut next_col: usize,
//    i: usize,
//    n: usize,
//    next_row: &mut HashMap<usize, Vec<usize>>,
//    mut v: Vec<usize>,
//) -> Vec<usize> {
//    if i == n {}
//}
// DP[r][c][state]
fn recurse(
    r: usize,
    c: usize,
    state: usize,
    dp: &mut Vec<Vec<usize>>,
    rows_map: &mut HashMap<usize, Vec<usize>>,
) -> usize {
    if dp[r][state] != usize::MAX {
        return dp[r][state];
    }
    if r == 1 {
        let mut a = usize::MAX;
        let mut b = usize::MAX;
        for i in 0..c {
            if state ^ (1 << i) == 0 {
                if a == usize::MAX {
                    a = i;
                } else {
                    b = i;
                }
                if b != usize::MAX {
                    if b == a + 1 {
                        a = usize::MAX;
                        b = usize::MAX;
                    } else {
                        a = b;
                    }
                }
            }
        }
        if a != b {
            dp[r][state] = 0;
            return 0;
        } else {
            dp[r][state] = 1;
            return 1;
        }
    }

    let next_row_states = next_row(state, c, rows_map).clone();

    dp[r][state] = next_row_states
        .iter()
        .map(|&next_row_state| recurse(r - 1, c, next_row_state, dp, rows_map))
        .sum();
    return dp[r][state];
}

fn f(r: usize, c: usize) -> usize {
    let mut rows_map = HashMap::new();
    let mut dp = vec![vec![usize::MAX; 1 << c]; r + 1];

    rows_map.insert((1 << c) - 1, vec![0]);

    let ret = recurse(r, c, 0, &mut dp, &mut rows_map);
    eprintln!("{rows_map:?}");
    ret
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    println!(
        "{}",
        f(
            s.split_whitespace().nth(0).unwrap().parse().unwrap(),
            s.split_whitespace().nth(0).unwrap().parse().unwrap(),
        )
    );
}

#[cfg(test)]
mod test_count_tilings {
    use super::f;

    #[test]
    #[ignore]
    fn example() {
        assert_eq!(f(4, 7), 781);
    }
}
