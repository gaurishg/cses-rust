use std::{io::Read, isize};

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    buffer.clear();

    let _ = std::io::stdin().read_to_string(&mut buffer);
    let mut movies: Vec<(isize, isize)> = buffer
        .lines()
        .map(|s| {
            let mut nums = s.split_ascii_whitespace();
            let begin = nums.next().unwrap().parse().unwrap();
            let end = nums.next().unwrap().parse().unwrap();
            (begin, end)
        })
        .collect();

    movies.sort_by(|(_, end1), (_, end2)| end1.cmp(end2));
    let mut max_movies = 0;
    let mut last_movie_ended = -1;
    for (begin, end) in movies {
        if begin >= last_movie_ended {
            last_movie_ended = end;
            max_movies += 1;
        }
    }
    println!("{max_movies}");
}
