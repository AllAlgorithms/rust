mod sort {
    pub fn merge_sort<T: Ord + Copy>(array: &mut [T]) {
        if array.len() < 2 {
            return;
        }
        let mid = (array.len() - 1) / 2;
        merge_sort(&mut array[0..=mid]);
        merge_sort(&mut array[(mid + 1)..]);
        merge(array, 0, mid, array.len() - 1);
    }

    fn merge<T: Ord + Copy>(array: &mut [T], mut start1: usize, mut mid: usize, end: usize) {
        let mut start2 = mid + 1;
        if array[mid] <= array[start2] {
            return;
        }
        while start1 <= mid && start2 <= end {
            if array[start1] <= array[start2] {
                start1 += 1;
            } else {
                let val = array[start2];
                let mut i = start2;
                while i != start1 {
                    array[i] = array[i - 1];
                    i -= 1;
                }
                array[start1] = val;
                start1 += 1;
                start2 += 1;
                mid += 1;
            }
        }
    }
}

fn main() {
    let mut array = [9, 4, 1, 7, 3, 2, 8, 5, 6];
    sort::merge_sort(&mut array);
    println!("{:?}", array);
}
