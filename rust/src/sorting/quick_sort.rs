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

    for j in low..pivot {
        if array[j] <= array[pivot] {
            array.swap(i, j);
            i+=1;
        }
    }
    array.swap(i, pivot);
    i
}


/*
*
* QUICK SORT
* (NO OUT OF BOUNDS)
*
*/
fn partition_oob(array: &mut Vec<u32>, low: usize, high: usize) -> usize {
    let pivot = high;
    let mut i = low;

    unsafe {
        let mut ptr = array.as_mut_ptr();
        for j in low..pivot {
            if array.get_unchecked(j) <= array.get_unchecked(pivot) {
                // array.swap(j, i);
                // https://doc.rust-lang.org/src/core/slice/mod.rs.html#960
                // there exists swap_unchecked, but in nightly builds only.
                // like insertion_sort, we can use raw pointers since that's
                // the negation of OOB under the hood.
                // 
                // This is like array.swap, which calls ptr::swap itself,
                // but we're ignoring the safety regulation of self[x] and self[y].
                std::ptr::swap(
                    ptr.add(i),
                    ptr.add(j)
                );
                i+=1;
            }
        }
        std::ptr::swap(
            ptr.add(i),
            ptr.add(pivot)
        );
    }
    i
}

/*
*
* QUICK SORT
* (RAW POINTERS)
*
*/
fn partition_rp(array: &mut Vec<u32>, low: usize, high: usize) -> usize {
    let pivot = high;
    let mut i = low;
    unsafe {
        let mut ptr = array.as_mut_ptr();
        for j in low..pivot {
            if *ptr.add(j) <= *ptr.add(pivot) {
                // swap(i, j+1)
                let tmp = *ptr.add(i);
                *ptr.add(i) = *ptr.add(j);
                *ptr.add(j) = tmp;

                i+= 1;
            }
        }
        // swap (i, pivot);
        let tmp = *ptr.add(i);
        *ptr.add(i) = *ptr.add(pivot);
        *ptr.add(pivot) = tmp;
    }
    i
}

/*
*
* COMMON BETWEEN SAFE AND UNSAFE
*
*/
#[inline(never)]
fn sort(array: &mut Vec<u32>, low: usize, high: usize, callable : fn(&mut Vec<u32>, usize, usize) -> usize) {
    if low >= high {
        return;
    }
    let mut pivot = 0;
    pivot = callable(array, low, high);
    sort(array, low, pivot-1, callable);
    sort(array, pivot+1, high, callable);
}


/*
* Primary entry point for Quick Sort function.
* Accepts a 2D array and benchmarks per iteration.
*
* If use_unsafe, it'll benchmark sort_unsafe and partition_unsafe.
*/
pub fn do_benchmark(array: &mut Vec<Vec<u32>>, method_type : &str) {
    // obviously we have to change things here compared to insert/bubble sort
    // ALL partition functions accept the array, low (usize), high (usize) and return the pivot (usize).
    let sort_method : fn(&mut Vec<u32>, usize, usize) -> usize = match method_type {
        "oob" => partition_oob,
        "rptr" => partition_rp,
        &_ => partition,
    };

    for mut sub_array in array.iter_mut() {
        benchmark(1, || sort(sub_array, 1, sub_array.len()-1, sort_method))
    }
}

// QUICK SORT TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut vec = vec![4, 7, 1, 3, 8, 6, 5, 2];
        let len = vec.len();
        sort(&mut vec, 0, len-1, partition);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_sort_oob() {
        let mut vec = vec![4, 7, 1, 3, 8, 6, 5, 2];
        let len = vec.len();
        sort(&mut vec, 0, len-1, partition_oob);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }


    #[test]
    fn test_sort_rptr() {
        let mut vec = vec![4, 7, 1, 3, 8, 6, 5, 2];
        let len = vec.len();
        sort(&mut vec, 0, len-1, partition_rp);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

}
