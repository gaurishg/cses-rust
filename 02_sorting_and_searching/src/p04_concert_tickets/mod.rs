use std::{collections::BTreeSet, isize};

fn f(tickets: Vec<usize>, customers: Vec<usize>) -> Vec<isize> {
    let mut tickets = tickets
        .into_iter()
        .enumerate()
        .map(|(idx, val)| (val, idx))
        .collect::<BTreeSet<_>>();

    customers
        .into_iter()
        .map(|c| {
            let mut x = tickets.range((
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded((c + 1, 0)),
            ));
            if let Some(&(ticket_price, idx)) = x.next_back() {
                tickets.remove(&(ticket_price, idx));
                ticket_price as isize
            } else {
                -1
            }
        })
        .collect()
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    buffer.clear();

    let _ = std::io::stdin().read_line(&mut buffer);
    let tickets = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    buffer.clear();

    let _ = std::io::stdin().read_line(&mut buffer);
    let customers = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for x in f(tickets, customers) {
        println!("{x}");
    }
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_1() {
        let tickets = vec![5, 3, 7, 8, 5];
        let customers = vec![4, 8, 3];
        let expected = vec![3, 8, -1];
        assert_eq!(expected, f(tickets, customers));
    }
}
