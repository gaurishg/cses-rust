use std::io::Read;

fn range_is_contained(range: &Vec<(i32, i32, usize)>) -> Vec<bool> {
    let mut range = range.clone();
    range.sort_by_key(|r| (r.0, -r.1));
    
    let mut res = vec![false; range.len()];
    let mut end = -1;
    for (_, stop, index) in range {
        if stop <= end {
            res[index] = true;
        } else {
            end = stop;
        }
    }
    
    res
}

fn range_contains(range: &Vec<(i32, i32, usize)>) -> Vec<bool> {
    let mut range = range.clone();
    range.sort_by_key(|r| (r.1, -r.0));
    
    let mut res = vec![false; range.len()];
    let mut begin = i32::MIN;

    for (start, _, index) in range {
        if start <= begin {
            res[index] = true;
        } else {
            begin = start;
        }
    }
    
    res
}

fn f(range: Vec<(i32, i32)>) -> (Vec<bool>, Vec<bool>) {
    let range = range.into_iter().enumerate().map(|(idx, (start, stop))| (start, stop, idx)).collect();
    (range_is_contained(&range), range_contains(&range))
}

pub fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    buf.clear();

    let _ = std::io::stdin().read_to_string(&mut buf);
    let range = buf.lines().map(|s| s.split_whitespace()).map(|mut it| {
      let x = it.next().unwrap().parse().unwrap();
      let y = it.next().unwrap().parse().unwrap();
      (x, y)  
    }).collect();

    let (contained, contains) = f(range);
    
    for x in contains {
        print!("{} ", x as u8);
    }
    println!();

    for x in contained {
        print!("{} ", x as u8);
    }
    println!();
}


#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_1() {
        let range = vec![(1, 6), (2, 4), (4, 8), (3, 6)];
        let contains = vec![true, false, false, false];
        let contained = vec![false, true, false, true];
        
        assert_eq!((contained, contains), f(range));
    }
}