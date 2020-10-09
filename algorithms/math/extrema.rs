mod extrema {
    pub fn min(array: &[i32]) -> i32 {
        if array.len() < 1 {
            return 0;
        }
        let mut minimum = array[0];
        for number in array {
            if minimum > *number {
                minimum = *number;
            }
        }
        return minimum;
    }
    pub fn max(array: &[i32]) -> i32 {
        if array.len() < 1 {
            return 0;
        }
        let mut maximum = array[0];
        for number in array {
            if maximum < *number {
                maximum = *number;
            }
        }
        return maximum;
    }
}

fn main() {
    let array = [1, 5, -3, 2, -1];
    println!("The minimum of {:?} is {}", array, exterma::min(&array));
    println!("The maximum of {:?} is {}", array, exterma::max(&array));
}