use std::collections::{BTreeSet, BinaryHeap};
fn is_valid(start: usize, end: usize, lights: &BTreeSet<usize>) -> bool {
    let mut it = lights.range((
        std::ops::Bound::Included(start),
        std::ops::Bound::Included(end),
    ));
    it.next();
    if let Some(&x) = it.next() {
        x == end
    } else {
        false
    }
}

fn f(x: usize, nums: Vec<usize>) -> Vec<usize> {
    let mut lights = BTreeSet::from([0, x]);

    // heap of (size, start, end)
    let mut heap = BinaryHeap::from([(x, 0, x)]);

    let mut result = vec![];

    for l in nums {
        let mut left_range =
            lights.range((std::ops::Bound::Unbounded, std::ops::Bound::Excluded(l)));
        let mut right_range =
            lights.range((std::ops::Bound::Excluded(l), std::ops::Bound::Unbounded));
        let left = left_range.next_back().copied().unwrap();
        let right = right_range.next().copied().unwrap();
        heap.push((l - left, left, l));
        heap.push((right - l, l, right));
        lights.insert(l);

        while let Some((size, start, end)) = heap.peek().copied() {
            if !is_valid(start, end, &lights) {
                heap.pop();
            } else {
                result.push(size);
                break;
            }
        }
    }
    result
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let x = buffer
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    buffer.clear();

    let _ = std::io::stdin().read_line(&mut buffer);
    let nums = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for ele in f(x, nums) {
        print!("{ele} ");
    }
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_1() {
        assert_eq!(vec![5, 3, 3], f(8, vec![3, 6, 2]));
    }
}
