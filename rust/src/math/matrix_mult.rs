
use crate::benchmark::benchmark;

/*
* Simple matrix multiplication--SAFE.
* Assumes that mat_a and mat_b are the same dimensions.
*/
fn matrix_mult(mat_a: &Vec<Vec<u32>>, mat_b: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let count = mat_a.len();
    let mut mat_c = vec![vec![0; count]; count];

    for i in 0..count {
        for j in 0..count {
            for k in 0..count {
                mat_c[i][j] += mat_a[i][k] * mat_b[k][j];
            }
        }
    }
    mat_c
}

/*
* Simple matrix multiplication--UNSAFE.
* Assumes that mat_a and mat_b are the same dimensions.
* (NO OUT OF BOUNDS)
*/
fn matrix_mult_oob(mat_a: &Vec<Vec<u32>>, mat_b: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let count = mat_a.len();
    let mut mat_c = vec![vec![0; count]; count];

    for i in 0..count {
        for j in 0..count {
            for k in 0..count {
                unsafe {
                    mat_c[i][j] += mat_a.get_unchecked(i).get_unchecked(k) * mat_b.get_unchecked(k).get_unchecked(j);
                }
            }
        }
    }
    mat_c
}


/*
* Simple matrix multiplication--UNSAFE.
* Assumes that mat_a and mat_b are the same dimensions.
* (RAW POINTERS)
*/
fn matrix_mult_rp(mat_a: &Vec<Vec<u32>>, mat_b: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let count = mat_a.len();
    let mut mat_c = vec![vec![0; count]; count];

    let mat_a_ptr = mat_a.as_ptr();
    let mat_b_ptr = mat_b.as_ptr();
    let mat_c_ptr = mat_c.as_mut_ptr();

    unsafe {
        for i in 0..count {
            let a_r = mat_a_ptr.add(i);
            let c_r = mat_c_ptr.add(i);
            for j in 0..count {
                let mut sum = 0;
                for k in 0..count {
                    let b_r = mat_b_ptr.add(k);
                    // we kinda have to do it this way because vec doesnt implement copy
                    // and we don't really wanna .clone()
                    sum += *(*a_r).as_ptr().add(k) * *(*b_r).as_ptr().add(j);
                }
                let c = (*c_r).as_mut_ptr().add(j);
                *c = sum;
            }
        }
    }
    mat_c
}

pub fn do_benchmark(
    benchmark_times : usize,
    mat_a : &Vec<Vec<u32>>,
    mat_b : &Vec<Vec<u32>>,
    method_type : &str) {

    let mat_method : fn(&Vec<Vec<u32>>, &Vec<Vec<u32>>) -> Vec<Vec<u32>> = match method_type {
        "oob" => matrix_mult_oob,
        "rptr" => matrix_mult_rp,
        &_ => matrix_mult,
    };

    benchmark(benchmark_times, || mat_method(mat_a, mat_b))
}


// MATRIX MULT TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mult() {
        let mut mat_a = vec![vec![1, 2], vec![3, 4]];
        let mut mat_b = vec![vec![5, 6], vec![7, 8]];
        assert_eq!(matrix_mult(&mut mat_a, &mut mat_b), vec![vec![19, 22], vec![43, 50]]);
    }

    #[test]
    fn test_mult_oob() {
        let mut mat_a = vec![vec![1, 2], vec![3, 4]];
        let mut mat_b = vec![vec![5, 6], vec![7, 8]];
        assert_eq!(matrix_mult_oob(&mut mat_a, &mut mat_b), vec![vec![19, 22], vec![43, 50]]);
    }

    #[test]
    fn test_mult_rptr() {
        let mut mat_a = vec![vec![1, 2], vec![3, 4]];
        let mut mat_b = vec![vec![5, 6], vec![7, 8]];
        assert_eq!(matrix_mult_rp(&mut mat_a, &mut mat_b), vec![vec![19, 22], vec![43, 50]]);
    }
}
