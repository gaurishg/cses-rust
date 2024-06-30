fn move_n_disks_from_a_to_c(n: usize, a: usize, b: usize, c: usize) {
    if n == 0 {
        return;
    } else if n == 1 {
        println!("{} {}", a, c);
        return;
    }
    move_n_disks_from_a_to_c(n - 1, a, c, b);
    move_n_disks_from_a_to_c(1, a, b, c);
    move_n_disks_from_a_to_c(n - 1, b, a, c);
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n = buffer.trim().parse().unwrap();
    println!("{}", (1 << n) - 1);
    move_n_disks_from_a_to_c(n, 1, 2, 3);
}
