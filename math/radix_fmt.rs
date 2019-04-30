fn main() {
    println!("{}", radix_fmt(42.345, 16).unwrap()); // 2A.585
    println!("{}", radix_fmt(5.5, 2).unwrap());     // 101.1
    println!("{}", radix_fmt(13.245, 8).unwrap());  // 15.1753412172
}

fn radix_fmt<T: Into<f64>>(number: T, obase: usize) -> Result<String, String> {
    if obase > 36 && obase < 2 {
        return Err(format!("Base {} is not supported!", obase));
    }
    let table: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let number: f64 = number.into();

    // format integral part of float
    let mut integral = number.trunc() as i64;
    let mut obase_int = String::new();
    while integral >= obase as i64 {
        obase_int.push(table[(integral % obase as i64) as usize]);
        integral /= obase as i64;
    }
    obase_int.push(table[integral as usize]);
    if number.is_sign_negative() {
        obase_int.push('-');
    }
    let obase_int = obase_int.chars().rev().collect::<String>();

    // format fractional part of float
    let mut fract = number.abs().fract();
    let mut obase_fract = String::new();
    let mut i = 0;
    loop {
        fract *= obase as f64;
        obase_fract.push(table[fract.trunc() as usize]);
        i += 1;
        if fract.fract() == 0. || i >= 15 {
            break;
        }
        fract = fract.fract();
    }

    Ok(format!("{}.{}", obase_int, obase_fract))
}
