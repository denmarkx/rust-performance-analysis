// rand = "0.9"

use rand::prelude::*;
use std::env;

// Provides an isolated example of a performance improvement
// from safe -> unsafe Rust.

// Utilizes all three wrapping_* functions:
// https://doc.rust-lang.org/std/intrinsics/fn.wrapping_mul.html
// https://doc.rust-lang.org/std/intrinsics/fn.wrapping_sub.html
// https://doc.rust-lang.org/std/intrinsics/fn.wrapping_add.html

// Despite these specific instriniscs not actually
// requiring unsafe, the actual isolated incident is 
// the way we access an element in the array.

include!{"../../../random_value.rs"}
include!{"../../../benchmark.rs"}
include!{"../../../util.rs"}

fn main() {
    let mut args : Vec<String> = env::args().collect();
    warn_arguments(&mut args);


    let array_size : usize = args[1].parse::<usize>().unwrap();
    let random_max : u32 = args[2].parse::<u32>().unwrap();
    let benchmark_count : usize = args[3].parse::<usize>().unwrap();


    let array = randomize_array(1, random_max, array_size);

    println!("\nw_mul_implicit:");
    benchmark(benchmark_count, || w_mul_implicit(&array));
    println!("\nw_mul_unsafe_implicit:");
    benchmark(benchmark_count, || w_mul_unsafe_implicit(&array));
    println!("\nw_mul:");
    benchmark(benchmark_count, || w_mul(&array, 0, array_size));
    println!("\nw_mul_unsafe:");
    benchmark(benchmark_count, || w_mul_unsafe(&array, 0, array_size));
}

fn w_mul_implicit(ar: &[u32]) {
    let mut n: u32 = 0;
    for i in 0..ar.len() {
        let x = ar[i];
        n = n.wrapping_mul(x);
    }
}


fn w_mul_unsafe_implicit(ar: &[u32]) {
    let mut n : u32 = 0;
    for i in 0..ar.len() {
        let x = unsafe { *ar.get_unchecked(i) };
        n = n.wrapping_mul(x);
    }
}


fn w_mul(ar: &[u32], min : usize, max : usize) {
    let mut n: u32 = 0;
    for i in min..max {
        let x = ar[i];
        n = n.wrapping_mul(x);
    }
}

fn w_mul_unsafe(ar: &[u32], min : usize, max : usize) {
    let mut n : u32 = 0;
    for i in min..max {
        let x = unsafe { *ar.get_unchecked(i) };
        n = n.wrapping_mul(x);
    }
}
