use crate::benchmark::benchmark;

#[inline(never)]
fn partition(array: &mut Vec<u32>, low: usize, high: usize) -> usize {
    let pivot = high;
    let mut i = low;

    for j in low..pivot-1 {
        if array[j] <= array[pivot] {
            array.swap(j, i);
            i+=1;
        }
    }
    array.swap(i, pivot);
    i
}

#[inline(never)]
fn sort(array: &mut Vec<u32>, low: usize, high: usize) {
    if low >= high {
        return;
    }
    let pivot = partition(array, low, high);
    sort(array, low, pivot-1);
    sort(array, pivot+1, high);
}

/*
* Primary entry point for Quick Sort function.
* Accepts a 2D array and benchmarks per iteration.
*/
pub fn start_sort(array: Vec<Vec<u32>>) {
    for sub_array in array.iter() {
        benchmark(1, || sort(&mut sub_array.clone(), 1, sub_array.len()-1));
    }
}
