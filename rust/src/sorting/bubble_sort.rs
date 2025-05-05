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
        for _i in 0..array.len() {
            for j in 0..array.len()-1 {
                if array.get_unchecked(j) > array.get_unchecked(j+1) {
                    // array.swap(j, j+1);
                    array.swap_unchecked(j, j+1);
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
                let a = ptr.add(j);
                let b = ptr.add(j + 1);
                if *a > *b {
                    std::ptr::swap(a, b);
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
