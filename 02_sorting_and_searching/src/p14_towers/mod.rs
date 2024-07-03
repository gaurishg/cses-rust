use std::collections::BTreeSet;

fn f(nums: Vec<isize>) -> usize {
    let mut towers = BTreeSet::<(isize, usize)>::new();
    for (idx, x) in nums.into_iter().enumerate() {
        let mut pos = towers.range((
            std::ops::Bound::Included((x + 1, 0)),
            std::ops::Bound::Unbounded,
        ));
        if let Some(ele) = pos.next().copied() {
            towers.remove(&ele);
        }
        towers.insert((x, idx));
    }

    towers.len()
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    buffer.clear();

    let _ = std::io::stdin().read_line(&mut buffer);
    println!(
        "{}",
        f(buffer
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect())
    );
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_1() {
        assert_eq!(2, f(vec![3, 8, 2, 1, 5]));
    }
}
