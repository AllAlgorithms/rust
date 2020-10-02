fn main() {
    // Usage:
    // ducci(a, b, c, d)
    ducci(123, 345, 456, 567);
}

fn ducci(a: i32, b: i32, c:i32, d:i32) {
    let mut n = (a, b, c, d);
    let mut counter = 2;
    loop {
        let old = n;
        n = ((n.0 - n.1).abs(), (n.2 - n.1).abs(), (n.3 - n.2).abs(), (n.0 - n.3).abs());

        if n == (0, 0, 0, 0) {
            break println!("{:?}", n)
        } else if n == old {
            break
        }

        println!("{:?}", n);
        counter += 1
    }

    println!("{}", counter);
}
