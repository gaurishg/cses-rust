use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Data((u64, u64));

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

fn f(t: u64, machine_times: Vec<u64>) -> u64 {
    let mut heap: BinaryHeap<Data> = machine_times.into_iter().map(|x| Data((x, x))).collect();

    let mut items_made = 0;
    let mut time_taken = 0;
    while items_made < t {
        let time_taken_for_this_item = heap.pop().unwrap();
        time_taken = std::cmp::max(time_taken, time_taken_for_this_item.0 .0);
        heap.push(Data((
            time_taken_for_this_item.0 .0 + time_taken_for_this_item.0 .1,
            time_taken_for_this_item.0 .1,
        )));
    }
    time_taken
}

pub fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);

    let t;
    {
        let mut it = buf.trim().split_whitespace();
        it.next();
        t = it.next().unwrap().parse().unwrap();
    }

    buf.clear();
    let _ = std::io::stdin().read_line(&mut buf);
    let machines = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();

    println!("{}", f(t, machines));
}
