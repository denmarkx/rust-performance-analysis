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
* (NO OUT OF BOUNDS CHECK)
*
*/
fn insertion_sort_oob(array: &mut Vec<u32>) {
    // Start with 2nd item:
    for i in 1..array.len() {
        unsafe {
            let n = *array.get_unchecked(i);
            let mut j = i;

            // Check previous:
            while j > 0 && *array.get_unchecked(j-1) > n {
                // Obviously we have to end up setting this at one point or another.
                // ..and we'll use raw pointeers, but the main gist here is all the unchecked calls.
                *array.get_unchecked_mut(j) = *array.get_unchecked(j-1); // equivalent to array[j] = ...
                j-=1;
            }

        // Insert front:
        *array.get_unchecked_mut(j) = n;
        }
    }
}

/*
*
* INSERTION SORT: UNSAFE
*   (RAW POINTERS)
*
*/
fn insertion_sort_rp(array: &mut Vec<u32>) {
    let x = array.as_mut_ptr();
    for i in 1..array.len() {
        let mut j = i;
        unsafe {
            while j > 0 && x.add(j-1) > x.add(j) {
                // we could use std::ptr::swap here, but it does a bit more copying.
                // https://doc.rust-lang.org/src/core/ptr/mod.rs.html#1009
                let tmp = *x.add(j);
                *x.add(j) = *x.add(j-1);
                j -= 1;
            }
        }
    }
}

/*
* start_sort: accepts 2D array and benchmarks per each iteration.
*/
pub fn do_benchmark(array: &mut Vec<Vec<u32>>, use_unsafe : bool) {
    for mut sub_array in array.iter_mut() {
        if use_unsafe {
            benchmark(1, || insertion_sort_oob(sub_array));
        } else {
            benchmark(1, || insertion_sort(sub_array));
        }
    }
}
