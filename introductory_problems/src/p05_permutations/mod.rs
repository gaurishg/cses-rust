pub fn f(n: u32) -> Result<Vec<u32>, &'static str> {
    match n {
        0 => Err("NO SOLUTION"),
        1 => Ok(vec![1]),
        2..=3 => Err("NO SOLUTION"),
        n if n % 4 == 0 => {
            let mut nums = vec![];
            for x in (1..n).step_by(4) {
                nums.push(x + 2);
                nums.push(x);
                nums.push(x + 3);
                nums.push(x + 1);
            }
            Ok(nums)
        }
        n if n % 4 == 1 => {
            let mut nums = vec![];
            nums.push(1);
            for x in (2..n).step_by(4) {
                nums.push(x + 2);
                nums.push(x);
                nums.push(x + 3);
                nums.push(x + 1);
            }
            Ok(nums)
        }
        n if n % 4 == 2 => {
            let mut nums = vec![];
            nums.push(1);
            for x in (2..n - 1).step_by(4) {
                nums.push(x + 2);
                nums.push(x);
                nums.push(x + 3);
                nums.push(x + 1);
            }
            nums.push(n);
            Ok(nums)
        }
        n if n % 4 == 3 => {
            let mut nums = vec![];
            nums.push(2);
            for x in (4..n).step_by(4) {
                nums.push(x + 2);
                nums.push(x);
                nums.push(x + 3);
                nums.push(x + 1);
            }
            nums.push(1);
            nums.push(3);
            Ok(nums)
        }
        _ => Err("Something happened which should have never happened"),
    }
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let buffer = buffer.trim();
    let n = buffer.parse();
    if let Err(e) = n.as_ref() {
        println!("{} -> {:?}", buffer, e);
    }
    let n = n.unwrap();
    match f(n) {
        Ok(nums) => {
            for x in nums {
                print!("{x} ");
            }
            println!();
        }
        Err(e) => println!("{e}"),
    }
}
