use std::io::Read;

const MAX_VAL: usize = 1_000_000_000 + 1;
struct RangeQuery {
    n: usize,
    data: Vec<usize>,
}

impl RangeQuery {
    pub fn new(n: usize) -> Self {
        Self{
            n: n.next_power_of_two(),
            data: vec![0; n.next_power_of_two() * 2]
        }
    }

    // pub fn from_vec(input_data: &Vec<usize>) -> Self {
    //     let data_size = input_data.len().next_power_of_two();
    //     let mut data = vec![0; 2 * data_size];
    //     for (idx, &x) in input_data.iter().enumerate() {
    //         data[data_size / 2 + idx] = x;
    //     }
    //     for i in (1..data_size).rev() {
    //         data[i] = data[2 * i] + data[2 * i + 1];
    //     }
    //     Self {
    //         n: data_size,
    //         data
    //     }
    // }
    

    fn sum_upto(&self, idx_in_tree: usize, query_l: usize, query_r: usize, current_l: usize, currrent_r: usize) -> usize {
        eprint!("idx: {}, q_l: {}, q_r: {}, l: {}, r: {}", idx_in_tree, query_l, query_r, current_l, currrent_r);
        if query_l > currrent_r || query_r < current_l {
            eprintln!(" :: SKIPPING");
            return 0;
        } else if current_l >= query_l && currrent_r <= query_r {
            eprintln!(" :: value: {}", self.data[idx_in_tree]);
            return self.data[idx_in_tree];
        }
        let last_of_left = (current_l + currrent_r) / 2;
        eprintln!(" :: one more round -> {} {} {}", current_l, last_of_left, currrent_r);
        return self.sum_upto(2 * idx_in_tree, query_l, query_r, current_l, last_of_left) + self.sum_upto(2 * idx_in_tree + 1, query_l, query_r, last_of_left + 1, currrent_r);
    }
    
    pub fn get_sum(&self, query_l: usize, query_r: usize) -> usize {
        eprintln!();
        return self.sum_upto(1, query_l, query_r, 0, self.n - 1);
    }
    
    pub fn add(&mut self, idx: usize) {
        let mut idx = self.n + idx;
        self.data[idx] += 1;
        idx /= 2;
        while idx > 0 {
            self.data[idx] = self.data[2 * idx] + self.data[2 * idx + 1];
            idx /= 2;
        }
    }
}

fn range_is_contained_in(range: &Vec<(i32, i32, usize)>) -> Vec<usize> {
    let mut range = range.clone();
    range.sort_by_key(|r|(r.0, -r.1));

    let mut range_query = RangeQuery::new(MAX_VAL);
    let mut result = vec![0; range.len()];
    for (_, stop, idx) in range {
        result[idx] = range_query.get_sum(stop as usize, MAX_VAL);
        range_query.add(stop as usize);
    }
    
    result
}



fn range_contains(range: &Vec<(i32, i32, usize)>) -> Vec<usize> {
    let mut range = range.clone();
    range.sort_by_key(|r|(r.1, -r.0));

    let mut res = vec![0; range.len()];
    let mut range_query = RangeQuery::new(MAX_VAL);
    for (start, _, idx) in range {
        res[idx] = range_query.get_sum(start as usize, MAX_VAL);
        range_query.add(start as usize);
    }
    res
}

fn f(range: Vec<(i32, i32)>) -> (Vec<usize>, Vec<usize>) {
    let range = range.into_iter().enumerate().map(|(index, (start, stop))| (start, stop, index)).collect();
    (range_contains(&range), range_is_contained_in(&range))
}

pub fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    buf.clear();

    let _ = std::io::stdin().read_to_string(&mut buf);
    let range = buf.lines().map(|s| {
        s.split_whitespace()
    }).map(|mut s|{
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        (x, y)  
    }).collect();
    let (v1, v2) = f(range);
    for x in v1 {
        print!("{x} ");
    }
    println!();

    for x in v2 {
        print!("{x} ");
    }
    println!();
}


#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_1() {
        let range = vec![(1, 6), (2, 4), (4, 8), (3, 6)];
        let contains_correct = vec![2, 0, 0, 0];
        let contained_correct = vec![0, 1, 0, 1];
        assert_eq!((contains_correct, contained_correct), f(range));
    }
}