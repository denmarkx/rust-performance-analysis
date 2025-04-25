use crate::benchmark::benchmark;

fn insertion_sort(array: &mut [u32]) {
    // Start with 2nd item:
    for i in 1..array.len() {
        let n = array[i];
        let mut j = i;

        // Check previous:
        while j > 0 && array[j-1] > n {
            array[j] = array[j-1];
            j-=1;
        }

        // Insert front:
        array[j] = n;
    }
}


/*
* start_sort: accepts 2D array and benchmarks per each iteration.
*/
pub fn start_sort(array: Vec<Vec<u32>>) {
    for sub_array in array.iter() {
        benchmark(1, || insertion_sort(&mut sub_array.clone()));
    }
}
