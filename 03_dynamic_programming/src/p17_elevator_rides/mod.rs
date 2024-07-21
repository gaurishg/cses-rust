use std::{collections::BTreeSet, io::Read};

// f(remaining, maxallowed, remainingpeople) = if any of remaining people <= remaining ::
// MIN[f(remaining-x, maxallowd, remainingpeople - x)]
// otherwise :: 1 + MIN[f(maxallowed - x, maxallowed, remainingpeople - x)]
fn f(max_allowed: usize, v: Vec<usize>) -> usize {
    let mut people: BTreeSet<_> = v.into_iter().collect();
    let mut rides = 0;

    while !people.is_empty() {
        rides += 1;
        let mut remaining = max_allowed;

        eprint!("In round {rides}, taking ");
        let mut round_total = 0;
        while let Some(&wt) = people.range(0..=remaining).last() {
            eprint!("{wt}, ");
            remaining -= wt;
            round_total += wt;
            people.remove(&wt);
        }

        eprintln!(":: round_total = {round_total}, remaining = {remaining}");
    }

    rides
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);
    println!(
        "{}",
        f(
            s.split_whitespace().nth(1).unwrap().parse().unwrap(),
            s.split_whitespace()
                .skip(2)
                .map(|s| s.parse().unwrap())
                .collect()
        )
    );
}

#[cfg(test)]
mod test_elevator_rides {
    use super::f;

    fn parse_string_and_call_f(s: &str) -> usize {
        f(
            s.split_whitespace().nth(1).unwrap().parse().unwrap(),
            s.split_whitespace()
                .skip(2)
                .map(|s| s.parse().unwrap())
                .collect(),
        )
    }

    #[test]
    #[ignore]
    fn example() {
        assert_eq!(f(10, vec![4, 8, 6, 1]), 2);
    }

    #[test]
    #[ignore]
    fn test_9() {
        let s = "20 1000000000
643426892 4189142 121707902 43875825 149981120 511121446 403139201 697420438 21368217 283420734 568341429 522280309 254956029 551977391 371486189 53782650 1391642 125720339 477357911 101617438";
        assert_eq!(parse_string_and_call_f(s), 6);
    }
}
