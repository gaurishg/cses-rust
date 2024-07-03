use std::{collections::BTreeSet, io::Read, isize};

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    //let n: isize = buffer.trim().parse().unwrap();

    buffer.clear();
    let _ = std::io::stdin().read_to_string(&mut buffer);
    let times: BTreeSet<(isize, isize)> = buffer
        .lines()
        .flat_map(|x| {
            let mut it = x.split_ascii_whitespace();
            let arrival = it.next().unwrap().parse().unwrap();
            let departure = it.next().unwrap().parse().unwrap();
            vec![(arrival, 1), (departure, -1)].into_iter()
        })
        .collect();
    let max_customers = times
        .into_iter()
        .map(|(_, v)| v)
        .fold((0, 0), |(curr_sum, max_sum), v| {
            (curr_sum + v, std::cmp::max(max_sum, curr_sum + v))
        })
        .1;
    println!("{max_customers}");
}
