mod sort {
    pub fn heap_sort<T: Ord + std::fmt::Debug>(array: &mut [T]) {
        println!("{:?}", array);
        let len = array.len();
        for i in (0..(len / 2)).rev() {
            heapify(array, len, i);
        }
        println!("Max-heap:");
        println!("{:?}", array);
        for i in (0..len).rev() {
            array.swap(0, i);
            heapify(array, i, 0);
        }
        println!("Result:");
        println!("{:?}", array);
    }

    fn heapify<T: Ord>(array: &mut [T], n: usize, i: usize) {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && array[left] > array[largest] {
            largest = left;
        }
        if right < n && array[right] > array[largest] {
            largest = right;
        }

        if largest != i {
            array.swap(i, largest);
            heapify(array, n, largest);
        }
    }

}

fn main() {
    let mut array = [9, 4, 1, 7, 3, 2, 8, 5, 6];
    // let mut array = [1, 12, 9, 5, 6, 10];
    sort::heap_sort(&mut array);
    println!("{:?}", array);
}
