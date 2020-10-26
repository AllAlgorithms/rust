mod sort {
    pub fn selection_sort<T: Ord + std::fmt::Debug>(array: &mut [T]) {
        for i in 0..array.len() {
            println!("{:?}", array);
            let min = search_min(&array[i..]) + i;
            if min != i {
                array.swap(i, min);
            }
        }
    }

    pub fn search_min<T: Ord + std::fmt::Debug>(array: &[T]) -> usize {
        let mut min = 0;
        for i in 0..array.len() {
            if array[min] > array[i] {
                min = i;
            }
        }
        min
    }
}

fn main() {
    let mut array = [9, 4, 1, 7, 3, 2, 8, 5, 6];
    sort::selection_sort(&mut array);
    println!("{:?}", array);
}
