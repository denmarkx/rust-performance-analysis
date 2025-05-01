
use crate::benchmark::benchmark;

/*
* Simple matrix multiplication--SAFE.
* Assumes that mat_a and mat_b are the same dimensions.
*/
fn matrix_mult(mat_a: &mut Vec<Vec<u32>>, mat_b: &mut Vec<Vec<u32>>) {
    let count = mat_a.len();
    let mut mat_c = vec![vec![0; count]; count];

    for i in 0..count {
        for j in 0..count {
            for k in 0..count {
                mat_c[i][j] += mat_a[i][k] * mat_b[k][j];
            }
        }
    }
}

/*
* Simple matrix multiplication--UNSAFE.
* Assumes that mat_a and mat_b are the same dimensions.
*/
fn matrix_mult_unsafe(mat_a: &mut Vec<Vec<u32>>, mat_b: &mut Vec<Vec<u32>>) {
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
}


/*
* Given a number of benchmark times and two matrices, benchmark per first argument.
* Final argument is use_unsafe: this'll benchmark matrix_mult_unsafe.
*/
pub fn do_benchmark(
	benchmark_times : usize,
	mut mat_a : &mut Vec<Vec<u32>>,
	mut mat_b : &mut Vec<Vec<u32>>,
	use_unsafe : bool) {

	// Choose which to benchmark:
	if use_unsafe {
    	benchmark(benchmark_times, || matrix_mult_unsafe(mat_a, mat_b));
    } else {
    	benchmark(benchmark_times, || matrix_mult(mat_a, mat_b));
    }
}
