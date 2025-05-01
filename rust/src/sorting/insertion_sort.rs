use crate::benchmark::benchmark;

/*
*
* INSERTION SORT: SAFE
*
*/
fn insertion_sort(array: &mut Vec<u32>) {
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
fn insertion_sort_unsafe(array: &mut Vec<u32>) {
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
pub fn do_benchmark(array: &mut Vec<Vec<u32>>, use_unsafe : bool) {
    for mut sub_array in array.iter_mut() {
        if use_unsafe {
            benchmark(1, || insertion_sort_unsafe(sub_array));
        } else {
            benchmark(1, || insertion_sort(sub_array));
        }
    }
}
