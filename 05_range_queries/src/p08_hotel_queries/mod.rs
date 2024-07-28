use std::io::Read;

struct SegmentTree {
    n: usize,
    tree: Vec<usize>,
}

impl SegmentTree {
    fn from(v: Vec<usize>) -> Self {
        let n = v.len().next_power_of_two();
        let mut tree = vec![usize::MIN; 2 * n];
        tree[n..n + v.len()].copy_from_slice(&v[..]);
        for i in (1..n).rev() {
            tree[i] = tree[2 * i].max(tree[2 * i + 1]);
        }

        Self { n, tree }
    }

    fn get_index_and_update(&mut self, rooms_required: usize) -> usize {
        if self.tree[1] < rooms_required {
            return 0;
        }

        let mut node = 1;
        while node < self.n {
            if self.tree[2 * node] >= rooms_required {
                node = 2 * node;
            } else {
                node = 2 * node + 1;
            }
        }

        let index = node - self.n + 1;
        self.tree[node] -= rooms_required;
        node /= 2;
        while node != 0 {
            self.tree[node] = self.tree[2 * node].max(self.tree[2 * node + 1]);
            node /= 2;
        }

        return index;
    }
}

fn f(rooms: Vec<usize>, tourists: Vec<usize>) -> Vec<usize> {
    let mut st = SegmentTree::from(rooms);
    tourists
        .into_iter()
        .map(|rooms_required| st.get_index_and_update(rooms_required))
        .collect()
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);
    let (hotels, tourists) = parse_input(&s);
    for x in f(hotels, tourists) {
        print!("{x} ");
    }
}

fn parse_input(s: &str) -> (Vec<usize>, Vec<usize>) {
    (
        s.lines()
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(),
        s.lines()
            .nth(2)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(),
    )
}

#[cfg(test)]
mod test_hotel_queries {
    use super::{f, parse_input};

    #[test]
    fn example() {
        let s = "8 5
3 2 4 1 5 5 2 6
4 4 7 1 1";
        let (rooms, guests) = parse_input(s);
        assert_eq!(f(rooms, guests), vec![3, 5, 0, 1, 1]);
    }
}
