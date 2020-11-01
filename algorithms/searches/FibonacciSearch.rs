use std::cmp::min;

pub fn fibonacci_search(array: &[i32], item: i32) -> i32 {
    let mut fb2 = 0;
    let mut fb1 = 1;
    let mut fb = fb2 + fb1;
    let mut offset: i32 = -1;
    while fb < array.len() {
        fb2 = fb1;
        fb1 = fb;
        fb = fb2 + fb1;
    }
    let mut i: i32;
    while fb > 1 {
        i = min(offset + fb2 as i32, array.len() as i32 - 1);
        if array[i as usize] < item {
            fb = fb1;
            fb1 = fb2;
            fb2 = fb - fb1;
            offset = i;
        } else if array[i as usize] > item {
            fb = fb2;
            fb1 = fb1 - fb2;
            fb2 = fb - fb1;
        } else {
            return i;
        }
    }
    if fb1 != 0 && array[offset as usize + 1] == item {
        return offset + 1;
    }
    -1
}

fn main() {
    let array = [1,2,3,4,5];
    println!("{}", fibonacci_search(&array, 1));
    println!("{}", fibonacci_search(&array, 2));
    println!("{}", fibonacci_search(&array, 3));
    println!("{}", fibonacci_search(&array, 4));
    println!("{}", fibonacci_search(&array, 5));
}
