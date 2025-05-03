// For a minimal example, see the root rust_asm/auto_vectorization.rs
// and rust_asm/asm_data/
use std::simd::Simd;
use std::simd::num::SimdUint;

use crate::benchmark::benchmark;

/*
*
* NO VECTORIZATION
*
*/
#[inline(never)]
fn update(data: &mut Vec<u32>, data2: &Vec<u32>) -> u32 {
    let mut n  = 0u32;
    for i in 1..data.len()/2 {
        data[i*2] += data2[i-1] * data[i] * 2;
        n += data[i];
    }
    n
}


/*
*
* VECTORIZATION VIA MANUAL SIMD
*
*/
#[inline(always)]
pub fn update_vectorization(data: &mut Vec<u32>, data2: &Vec<u32>) -> u32 {
    const PIPES: usize = 8;
    type SimdU32 = Simd<u32, PIPES>;

    let len = data.len() / 2;
    let simd_len = len - (len % PIPES);

    let data_ptr = data.as_mut_ptr();
    let data2_ptr = data2.as_ptr();

    let mut n = 0u32;
    let mut i = 1;

    unsafe {
        while i + PIPES <= simd_len {
            // unroll 1-8
            let mut v_data = SimdU32::from_array([
                *data_ptr.add(i + 0),
                *data_ptr.add(i + 1),
                *data_ptr.add(i + 2),
                *data_ptr.add(i + 3),
                *data_ptr.add(i + 4),
                *data_ptr.add(i + 5),
                *data_ptr.add(i + 6),
                *data_ptr.add(i + 7),
            ]);

            // unroll
            let v_data2 = SimdU32::from_array([
                *data2_ptr.add(i - 1 + 0),
                *data2_ptr.add(i - 1 + 1),
                *data2_ptr.add(i - 1 + 2),
                *data2_ptr.add(i - 1 + 3),
                *data2_ptr.add(i - 1 + 4),
                *data2_ptr.add(i - 1 + 5),
                *data2_ptr.add(i - 1 + 6),
                *data2_ptr.add(i - 1 + 7),
            ]);

            let v_result = v_data * v_data2 * SimdU32::splat(2);

            for j in 0..PIPES {
                let dst_idx = (i + j) * 2;
                if dst_idx < data.len() {
                    *data_ptr.add(dst_idx) += v_result[j];
                }
            }

            n += v_data.reduce_sum();
            i += PIPES;
        }

        while i < len {
            let val = *data_ptr.add(i);
            *data_ptr.add(i * 2) += *data2_ptr.add(i - 1) * val * 2;
            n += val;
            i += 1;
        }
    }

    n
}

pub fn do_benchmark(array: &mut Vec<Vec<u32>>, method_type : &str) {
    let method : fn(&mut Vec<u32>, &Vec<u32>) -> u32 = match method_type {
        "vectorization" => update_vectorization,
        &_ => update,
    };

    for sub_array in array.iter_mut() {
        benchmark(1, || method(sub_array, &sub_array.clone()))
    }
}
