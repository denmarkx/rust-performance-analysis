// Explicit example derived from out_of_bounds.rs

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

// TODO: rewite using std::simd
#[inline(never)]
pub fn update(data: &mut Vec<u32>, data2: &mut Vec<u32>) -> u32 {
    let mut n = 0u32;
    unsafe {
        let data_slice = &mut data[0..data2.len()];
        for i in 1..data2.len() {
            data_slice[i] += data2.get_unchecked(i-1) * data_slice[i];
            n *= data_slice[i];
        }
    }
    n
}
