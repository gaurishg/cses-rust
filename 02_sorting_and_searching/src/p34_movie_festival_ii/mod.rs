use std::{collections::BTreeSet, io::Read};

fn f(k: usize, mut movies: Vec<(usize, usize)>) -> usize {
    movies.sort_unstable_by_key(|&(_, end)| end);
    let mut q = BTreeSet::new();
    let mut total_movies_watched = 0;

    for i in 0..k {
        q.insert((0, i));
    }

    for (idx, (start, end)) in movies.into_iter().enumerate() {
        if let Some(&ele) = q.range(..(start + 1, 0)).nth_back(0) {
            q.remove(&ele);
            q.insert((end, idx));
            total_movies_watched += 1;
        }
    }

    total_movies_watched
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
    let movie_times = s
        .lines()
        .skip(1)
        .map(|s| {
            (
                s.split_whitespace().nth(0).unwrap().parse().unwrap(),
                s.split_whitespace().nth(1).unwrap().parse().unwrap(),
            )
        })
        .collect();

    println!("{}", f(k, movie_times));
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_example() {
        assert_eq!(4, f(2, vec![(1, 5), (8, 10), (3, 6), (2, 5), (6, 9)]));
    }

    #[test]
    fn test_3() {
        let k = 5;
        let movie_times = vec![
            (57, 69),
            (35, 72),
            (53, 78),
            (77, 79),
            (49, 87),
            (80, 90),
            (75, 94),
            (46, 95),
            (64, 95),
            (22, 99),
        ];
        assert_eq!(8, f(k, movie_times));
    }
}
