pub fn linear_search(array: &[i32], item: i32) -> i32 {
    for i in 0..array.len() {
        if item == array[i] {
            return i as i32;
        }
    }
    return -1;
}

fn main() {
    let array = [1,2,3,4,5];
    println!("{}", linear_search(&array, 3));
}