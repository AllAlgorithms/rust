mod search {
    pub fn exponential_search(array: &[i32], item: i32) -> i32 {
        if array.len() == 0 {
            return -1;
        }
        let mut i = 1;
        while i < array.len() {
            if array[i] < item {
                i *= 2;
            } else {
                break;
            }
        }
        if i >= array.len() {
            i = array.len() - 1;
        }
        let index = binary_search(&array[(i/2)..=i], item);
        if index != -1 { index + (i as i32 / 2) } else { -1 }
    }

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
	    -1
    }
}

fn main() {
    let arr = [ 1, 2, 5, 11, 12, 15, 21, 22, 25 ];
    println!("{}", search::exponential_search(&arr, 1));
    println!("{}", search::exponential_search(&arr, 5));
    println!("{}", search::exponential_search(&arr, 15));
    println!("{}", search::exponential_search(&arr, 22));
    println!("{}", search::exponential_search(&arr, 25));
    println!("{}", search::exponential_search(&arr, 45));
    println!("{}", search::exponential_search(&arr, -45));
}
