mod sort {
    pub fn insertion_sort<T: Ord + std::fmt::Debug>(array: &mut [T]) {
        let mut n;
        for i in 0..array.len() {
            println!("{:?}", array);
            n = i;
            while n > 0 && array[n] < array[n - 1] {
                array.swap(n, n - 1);
                n = n - 1;
            }
        }
    }
}

fn main() {
    let mut array = [9, 4, 1, 7, 3, 2, 8, 5, 6];
    sort::insertion_sort(&mut array);
    println!("{:?}", array);
}
