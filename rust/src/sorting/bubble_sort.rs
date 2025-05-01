use crate::benchmark::benchmark;

/*
*
* BUBBLE SORT SAFE
*
*/
#[inline(never)]
fn bubble_sort(array: &mut Vec<u32>) {
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
*
* BUBBLE SORT UNSAFE
* USE OF RAW POINTERS!
* This was a change for the presentation's final slides as I
# ended up changing this to get the data vs making a new function.
* ..will need to re-adapt and split up later.
*
*/
fn bubble_sort_unsafe(array: &mut Vec<u32>) {
    // Bubble Sort Implementation:
    unsafe {
        let mut ptr = array.as_mut_ptr();

        for _i in 0..array.len() {
            for j in 0..array.len()-1 {
                if *ptr.add(j) > *ptr.add(j + 1) {
                    // swap:
                    ptr.add(j).swap(ptr.add(j + 1));
                }
            }
        }
    }
}


/*
* start_sort: accepts 2D array and benchmarks per each iteration.
*/
pub fn do_benchmark(array: &mut Vec<Vec<u32>>, use_unsafe : bool) {
    for sub_array in array.iter_mut() {
        if use_unsafe {
            benchmark(1, || bubble_sort_unsafe(sub_array));
        } else {
            benchmark(1, || bubble_sort(sub_array));
        }
    }
}
