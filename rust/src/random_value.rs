use rand::Rng;

/*
* Linear Congruential Generator
* 
* Naive implementation for the sole use of providing a seed and having it
* generate the same values independent of language.
*/
fn lcg(seed: &mut u32) -> u32 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed
}

fn lcg_rand_range(seed: &mut u32, min: u32, max: u32) -> u32 {
    let r = lcg(seed);
    (r + min) % (max - min) + min
}

/*
* Given three parameters, a, b, c, returns a 2D vector of randomized values.
* a (u32), b (u32) -- random_range(a..b+1) for each element in inner vector.
* c (usize): Number of arrays.
*
* The length of the inner array is determined by:
*   [0..random_range(b / 2..=b)].
*   ..IF inner_length is 0.
*   ..otherwise, it will use whatever was passed to inner_length
*
* if seed > 0, the LCG random num generator is used. Otherwise,
* the rand::Rng crate is.
*/
pub fn randomize_array_set(a : u32, b : u32,
    mut c : usize, inner_length : usize, seed: &mut u32) -> Vec<Vec<u32>> {
    // c will change to inner_length if inner_length is not 0.
    if inner_length != 0 {
        c = inner_length;
    }

    // seed will be changed in-place, so we'll copy the real seed.
    let o_seed = *seed;

    let mut rng;
    let mut length = c;

    // Create c arrays
    let mut array: Vec<Vec<u32>> = vec![];
    let mut rng_gen = rand::rng();

    // Insert into array:
    // We separate these to be in parity with C's code.
    let mut length_vec = vec![0; c];
    for i in 0..c {
        if inner_length == 0 {
            if o_seed == 0 {
                length = rng_gen.random_range(b / 2..=b) as usize;
            } else {
                length = lcg_rand_range(seed, b / 2, b) as usize;
            }
        }
        length_vec[i] = length;
    }

    // Insert into array:
    for i in 0..c {
        let arr_length = length_vec[i];
        let mut vec = vec![0; arr_length];
        for j in 0..arr_length {
            if o_seed == 0 {
                rng = rng_gen.random_range(a..b+1);
                vec[j] = rng.try_into().unwrap();
            } else {
                vec[j] = lcg_rand_range(seed, a, b+1);
            }
        }
        array.push(vec);
    }
    array
}
