use std::io::Read;

fn f(mut books: Vec<u64>) -> u64 {
    books.sort_unstable();
    let total: u64 = books.iter().sum();
    std::cmp::max(total, 2 * books.last().unwrap())
}

pub fn main() {
    let mut buf = String::new();
    _ = std::io::stdin().read_to_string(&mut buf);

    let books = buf
        .lines()
        .skip(1)
        .flat_map(|s| s.split_whitespace().map(|x| x.parse().unwrap()))
        .collect();
    println!("{}", f(books));
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_1() {
        assert_eq!(16, f(vec![2, 8, 3]));
    }
}
