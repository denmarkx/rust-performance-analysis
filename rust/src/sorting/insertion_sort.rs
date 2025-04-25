use crate::benchmark::benchmark;

/*
*
* INSERTION SORT: SAFE
*
*/
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
*
* INSERTION SORT: UNSAFE
*
*/
fn insertion_sort_unsafe(array: &mut [u32]) {
    // Start with 2nd item:
    unsafe {
        for i in 1..array.len() {
            let n = *array.get_unchecked(i);
            let mut j = i;

            // Check previous:
            while j > 0 && *array.get_unchecked(j-1) > n {
                array[j] = *array.get_unchecked(j-1);
                j-=1;
            }

            // Insert front:
            array[j] = n;
        }
    }
}

/*
* start_sort: accepts 2D array and benchmarks per each iteration.
*/
pub fn do_benchmark(array: Vec<Vec<u32>>, use_unsafe : bool) {
    for sub_array in array.iter() {
        if use_unsafe {
            benchmark(1, || insertion_sort_unsafe(&mut sub_array.clone()));
        } else {
            benchmark(1, || insertion_sort(&mut sub_array.clone()));
        }
    }
}
