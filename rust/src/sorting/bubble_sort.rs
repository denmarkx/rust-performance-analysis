use crate::benchmark::benchmark;

#[inline(never)]
fn bubble_sort(array: &mut[u32]) {
    // Bubble Sort Implementation:
    for _i in 0..array.len() {
        for j in 0..array.len()-1 {
            if array[j] > array[j+1] {
                // swap:
                array.swap(j, j+1);
            }
        }
    }
}

/*
* do_sort: accepts 2D array and benchmarks per each iteration.
*/
pub fn do_sort(array: Vec<Vec<u32>>) {
    for sub_array in array.iter() {
        benchmark(1, || bubble_sort(&mut sub_array.clone()));
    }
}
