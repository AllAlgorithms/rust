pub fn binary_search(array: &[i32], item: i32) -> i32 {
    let mut high: i32 = array.len() as i32 - 1;
    let mut low: i32 = 0;
    let mut mid: i32;
	while low <= high {
		mid = ((high - low) / 2) + low;
		let val = array[mid as usize];
		if val == item {
			return mid as i32;
		}
		if val > item {
			high = mid - 1;
		}
		if val < item {
			low = mid + 1;
		}	
	}
	return -1;
}

fn main() {
    let arr = [ 1, 2, 5, 11, 12, 15, 21, 22, 25 ];
    println!("{}",binary_search(&arr, 15));
}
