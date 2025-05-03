// Explicit example derived from out_of_bounds.rs

#[inline(never)]
#[cfg(not(vectorization))]
// no simd instructions.
pub fn update(data: &Vec<u32>, data2: &Vec<u32>) -> u32 {
    let mut n = 0u32;
    for i in 1..data.len() {
        n += data[i] * data2[i-1];
    }
    n
}

#[inline(never)]
#[cfg(vectorization)]
// movdqa
pub fn update(data: &Vec<u32>, data2: &Vec<u32>) -> u32 {
    let mut n = 0u32;
    for i in 1..data.len() {
        n += unsafe { data.get_unchecked(i) * data2.get_unchecked(i-1) };
    }
    n
}
