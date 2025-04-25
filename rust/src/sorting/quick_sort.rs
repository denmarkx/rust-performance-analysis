use crate::benchmark::benchmark;

/*
*
* SAFE QUICK SORT
*
*/
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



/*
*
* UNSAFE QUICK SORT
*
*/
fn partition_unsafe(array: &mut Vec<u32>, low: usize, high: usize) -> usize {
    let pivot = high;
    let mut i = low;

    for j in low..pivot-1 {
        unsafe {
            if array.get_unchecked(j) <= array.get_unchecked(pivot) {
                array.swap(j, i);
                i+=1;
            }
        }
    }
    array.swap(i, pivot);
    i
}

/*
*
* COMMON BETWEEN SAFE AND UNSAFE
* XXX: Unknown if that extra conditional for use_unsafe we're adding
* impacts performance. We're gonna assume not at the moment.
*/
#[inline(never)]
fn sort(array: &mut Vec<u32>, low: usize, high: usize, use_unsafe : bool) {
    if low >= high {
        return;
    }
    let mut pivot = 0;
    if use_unsafe {
        pivot = partition_unsafe(array, low, high);
    } else {
        pivot = partition(array, low, high);
    }
    sort(array, low, pivot-1, use_unsafe);
    sort(array, pivot+1, high, use_unsafe);
}


/*
* Primary entry point for Quick Sort function.
* Accepts a 2D array and benchmarks per iteration.
*
* If use_unsafe, it'll benchmark sort_unsafe and partition_unsafe.
*/
pub fn do_benchmark(array: Vec<Vec<u32>>, use_unsafe : bool) {
    for sub_array in array.iter() {
        benchmark(1, || sort(&mut sub_array.clone(), 1, sub_array.len()-1, use_unsafe));
    }
}
