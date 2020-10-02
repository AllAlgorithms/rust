mod sort {
    pub fn bubblesort<T: Ord + std::fmt::Debug>(array: &mut [T]) {
        for i in 0..array.len() {
            println!("{:?}", array);
            let ending = array.len() - i;
            bubblesort_step(&mut array[..ending]);
        }
    }

    fn bubblesort_step<T: Ord>(array: &mut [T]) {
        for i in 0..array.len() {
            if i == array.len() - 1 {
                return;
            }

            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
            }
        }
    }
}

fn main() {
    let mut array = [9, 4, 1, 7, 3, 2, 8, 5, 6];
    sort::bubblesort(&mut array);
    println!("{:?}", array);
}
