fn main() {
    println!("{}", ackermann(1, 1)); // 3
    println!("{}", ackermann(3, 4)); // 125
}

fn ackermann(m: u128, n: u128) -> u128 {
    if m == 0{
        n + 1
    } else if m > 0 && n == 0 {
        ackermann(m - 1, 1)
    } else if m > 0 && n > 0 {
        ackermann(m - 1, ackermann(m, n - 1))
    } else {
        0u128
    }
}
