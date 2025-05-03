#![feature(portable_simd)]

// Explicit example derived from out_of_bounds.rs
// Requires nightly!
use std::simd::Simd;
use std::simd::num::SimdUint;

#[inline(never)]
#[cfg(not(vectorization))]
pub fn update(data: &mut Vec<u32>, data2: &Vec<u32>) -> u32 {
    let mut n  = 0u32;
    for i in 1..data.len()/2 {
        data[i*2] += data2[i-1] * data[i] * 2;
        n += data[i];
    }
    n
}


#[inline(never)]
#[cfg(vectorization)]
pub fn update(data: &mut Vec<u32>, data2: &Vec<u32>) -> u32 {
    const PIPES: usize = 8;
    type SimdU32 = Simd<u32, PIPES>;
    let len = data.len() / 2;
    let simd_len = len - (len % PIPES);

    let mut n = 0u32;
    let mut i = 1;

    while i + PIPES <= simd_len {
        let idxs: [usize; PIPES] = core::array::from_fn(|j| i + j);
        let simd_idxs = Simd::from_array(idxs);

        let data_i = Simd::gather_or_default(data, simd_idxs);
        let data2_i = Simd::gather_or_default(data2, simd_idxs - Simd::splat(1));
        let data2x = data2_i * data_i * Simd::splat(2);

        let write_idxs = idxs.map(|j| j * 2);
        for j in 0..PIPES {
            if write_idxs[j] < data.len() {
                data[write_idxs[j]] += data2x[j];
            }
        }

        n += data_i.reduce_sum();

        i += PIPES;
    }

    while i < len {
        data[i * 2] += data2[i - 1] * data[i] * 2;
        n += data[i];
        i += 1;
    }

    n
}

// fn main() {
//     let mut test : Vec<u32> = vec![1, 2, 3, 4];   
//     let test2 : Vec<u32> = vec![1, 2, 3, 4]; 
//     dbg!(update(&mut test, &test2));
// }
