fn f(max_wt: usize, mut wts: Vec<usize>) -> usize {
    wts.sort();
    wts.truncate(wts.partition_point(|&wt| wt <= max_wt));

    let mut it = wts.iter().peekable();

    let mut count = 0;

    while it.peek().is_some() {
        count += 1;
        let last = it.next_back().unwrap().clone();
        if it.peek().is_none() {
            break;
        }

        let first = **it.peek().unwrap();
        if first + last <= max_wt {
            it.next();
        }
    }

    count
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let split = buffer.split_ascii_whitespace();
    //let n_children = split.next().unwrap().parse().unwrap();
    let max_wt = split.last().unwrap().parse().unwrap();

    buffer.clear();
    let _ = std::io::stdin().read_line(&mut buffer);
    let wts = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", f(max_wt, wts));
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_1() {
        assert_eq!(3, f(10, vec![7, 2, 3, 9]));
    }
}
