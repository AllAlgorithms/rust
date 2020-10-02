fn main() {
    let count: u32 = 0;
    let n: u32 = 8;
    println!("{} took {} steps", n, collatz(n, count));
}


fn collatz(n: u32, mut count: u32) -> u32 {
    if n == 1 {
        return count
    } else if n % 2 == 1 {
        count += 1;
        return collatz(3*n + 1, count)
    } else {
        count += 1;
        return collatz(n/2, count)
    }
}
