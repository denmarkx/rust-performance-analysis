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
* (NO OUT OF BOUNDS)
*
*/
#[inline(never)]
fn bubble_sort_oob(array: &mut Vec<u32>) {
    // Bubble Sort Implementation:
    unsafe {
        let ptr = array.as_mut_ptr();
        for _i in 0..array.len() {
            for j in 0..array.len()-1 {
                if array.get_unchecked(j) > array.get_unchecked(j+1) {
                    // array.swap(j, j+1);
                    // https://doc.rust-lang.org/src/core/slice/mod.rs.html#960
                    // there exists swap_unchecked, but in nightly builds only.
                    // like insertion_sort, we can use raw pointers since that's
                    // the negation of OOB under the hood.
                    // 
                    // This is like array.swap, which calls ptr::swap itself,
                    // but we're ignoring the safety regulation of self[x] and self[y].
                    std::ptr::swap(
                        ptr.add(j),
                        ptr.add(j+1)
                    )
                }
            }
        }
    }
}

/*
*
* BUBBLE SORT UNSAFE
*  (RAW POINTERS)
*
*/
#[inline(never)]
fn bubble_sort_rp(array: &mut Vec<u32>) {
    // Bubble Sort Implementation:
    unsafe {
        let mut ptr = array.as_mut_ptr();

        for _i in 0..array.len() {
            for j in 0..array.len()-1 {
                if *ptr.add(j) > *ptr.add(j + 1) {
                    // Previously we did ptr::swap, but we'll try and now let that be OOB's thing.
                    // See also: https://doc.rust-lang.org/src/core/ptr/mod.rs.html#1009
                    let tmp = *ptr.add(j);
                    *ptr.add(j) = *ptr.add(j+1);
                    *ptr.add(j+1) = tmp;
                }
            }
        }
    }
}

pub fn do_benchmark(array: &mut Vec<Vec<u32>>, method_type : &str) {
    let sort_method : fn(&mut Vec<u32>) = match method_type {
        "oob" => bubble_sort_oob,
        "rptr" => bubble_sort_rp,
        &_ => bubble_sort,
    };

    for mut sub_array in array.iter_mut() {
        benchmark(1, || sort_method(sub_array))
    }
}

// QUICK SORT TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut vec = vec![4, 7, 1, 3, 8, 6, 5, 2];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_sort_oob() {
        let mut vec = vec![4, 7, 1, 3, 8, 6, 5, 2];
        bubble_sort_oob(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }


    #[test]
    fn test_sort_rptr() {
        let mut vec = vec![4, 7, 1, 3, 8, 6, 5, 2];
        bubble_sort_rp(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
