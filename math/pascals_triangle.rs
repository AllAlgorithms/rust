/* Pascals Triangle
 *
 * NerdyPepper - nerdypepper at tuta dot io - 2019
 *
 */

fn main() {
    println!("{}", pascal_triangle(10));
}

fn binomial_coeff(n: u32, k: u32) -> u32 {
    // C(n, k) = (n)(n-1)(n-2)...(n-k+1) / (k)(k-1)(k-2)...1
    let mut coeff = 1u32;
    let mut r = k;
    if k > n - k {
        r = n - k;
    }
    for i in 0..r {
        coeff *= n - i;
        coeff /= i + 1;
    }
    coeff
}

fn pascal_triangle(n: u32) -> String {
    let mut triangle = String::new();
    for i in 0..n {
        for _ in 0..(n - i - 1)/2 * 5 {
            triangle.push_str(" ");
        }
        for j in 0..i + 1 {
            triangle.push_str(&format!("{:4} ", binomial_coeff(i, j)));
        }
        triangle.push_str("\n");
    }
    triangle
}
