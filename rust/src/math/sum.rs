// For a minimal example, see the root rust_asm/auto_vectorization.rs
// and rust_asm/asm_data/

use crate::benchmark::benchmark;

/*
*
* SUM: NO VECTORIZATION
*
*/
#[inline(never)]
fn sum(data: &Vec<u32>, data2: &Vec<u32>) -> u32 {
    let mut n = 0u32;
    for i in 1..data.len() {
        n += data[i] + data2[i-1];
    }
    n
}

/*
*
* SUM: VECTORIZATION
*
*/
#[inline(never)]
fn sum_vectorization(data: &Vec<u32>, data2: &Vec<u32>) -> u32 {
    let mut n = 0u32;
    for i in 1..data.len() {
        // I'm aware the only real difference is that this is an OOB check,
        // and the compiler can use SIMD instructions to auto-vec this,
        // but the point is to show speed without sorting.
        // ..
        // the sorting/.._oob functions sort a vector in place and we're not
        // interacting with two value-different (well not really [see: do_benchmark], but the
        // compiler doesn't know that from this function) containers.
        n += unsafe { data.get_unchecked(i) + data2.get_unchecked(i-1) };
    }
    n
}

pub fn do_benchmark(array: &Vec<Vec<u32>>, method_type : &str) {
    let method : fn(&Vec<u32>, &Vec<u32>) -> u32 = match method_type {
        "vectorization" => sum_vectorization,
        &_ => sum,
    };

    for sub_array in array.iter() {
        benchmark(1, || method(sub_array, sub_array))
    }
}
