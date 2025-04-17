// rand = "0.9"

use rand::prelude::*;
use std::env;

// Provides an isolated example of a performance improvement
// from safe -> unsafe Rust.

// Utilizes all three wrapping_* functions:
// https://doc.rust-lang.org/std/primitive.u32.html#method.wrapping_mul
// https://doc.rust-lang.org/std/primitive.u32.html#method.wrapping_sub
// https://doc.rust-lang.org/std/primitive.u32.html#method.wrapping_add

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

    // t_wrapping_mul:
    println!("\nw_func_implicit: [MUL]");
    benchmark(benchmark_count, || w_func_implicit(&array, t_wrapping_mul));

    println!("\nw_func_unsafe_implicit: [MUL]:");
    benchmark(benchmark_count, || w_func_unsafe_implicit(&array, t_wrapping_mul));

    println!("\nw_func: [MUL]:");
    benchmark(benchmark_count, || w_func(&array, 0, array_size, t_wrapping_mul));

    println!("\nw_func_unsafe: [MUL]:");
    benchmark(benchmark_count, || w_func_unsafe(&array, 0, array_size, t_wrapping_mul));

    println!("======================================");

    // t_wrapping_add:
    println!("\nw_func_implicit: [MUL]");
    benchmark(benchmark_count, || w_func_implicit(&array, t_wrapping_add));

    println!("\nw_func_unsafe_implicit: [MUL]:");
    benchmark(benchmark_count, || w_func_unsafe_implicit(&array, t_wrapping_add));

    println!("\nw_func: [MUL]:");
    benchmark(benchmark_count, || w_func(&array, 0, array_size, t_wrapping_add));

    println!("\nw_func_unsafe: [MUL]:");
    benchmark(benchmark_count, || w_func_unsafe(&array, 0, array_size, t_wrapping_add));

    println!("======================================");

    // t_wrapping_sub:
    println!("\nw_func_implicit: [MUL]");
    benchmark(benchmark_count, || w_func_implicit(&array, t_wrapping_sub));

    println!("\nw_func_unsafe_implicit: [MUL]:");
    benchmark(benchmark_count, || w_func_unsafe_implicit(&array, t_wrapping_sub));

    println!("\nw_func: [MUL]:");
    benchmark(benchmark_count, || w_func(&array, 0, array_size, t_wrapping_sub));

    println!("\nw_func_unsafe: [MUL]:");
    benchmark(benchmark_count, || w_func_unsafe(&array, 0, array_size, t_wrapping_sub));

}

/*
* Equivalent to n.wrapping_mul(x);
*/
#[inline]
fn t_wrapping_mul(n: &u32, x: &u32) -> u32 {
    n.wrapping_mul(*x)
}

/*
* Equivalent to n.wrapping_add(x);
*/
#[inline]
fn t_wrapping_add(n: &u32, x: &u32) -> u32 {
    n.wrapping_add(*x)
}

/*
* Equivalent to n.wrapping_sub(x);
*/
#[inline]
fn t_wrapping_sub(n: &u32, x: &u32) -> u32 {
    n.wrapping_sub(*x)
}

/*
*
* Retrieves index in function.
* w_func_implicit
* w_func_unsafe_implicit
*
*
*/
fn w_func_implicit(ar: &[u32], f: fn(&u32, &u32) -> u32) -> u32 {
    let mut n: u32 = 0;
    for i in 0..ar.len() {
        let x = ar[i];
        n = f(&n, &x)
    }
    n
}

fn w_func_unsafe_implicit(ar: &[u32], f: fn(&u32, &u32) -> u32) -> u32 {
    let mut n : u32 = 0;
    for i in 0..ar.len() {
        let x = unsafe { *ar.get_unchecked(i) };
        n = f(&n, &x);
    }
    n
}


/*
*
* Expects lower and upper array bounds in call.
* w_func
* w_func_unsafe
*
*
*/
fn w_func(ar: &[u32], min : usize, max: usize, f: fn(&u32, &u32) -> u32) -> u32 {
    let mut n: u32 = 0;
    for i in min..max {
        let x = ar[i];
        n = f(&n, &x);
    }
    n
}

fn w_func_unsafe(ar: &[u32], min : usize, max: usize, f: fn(&u32, &u32) -> u32) -> u32 {
    let mut n : u32 = 0;
    for i in min..max {
        let x = unsafe { *ar.get_unchecked(i) };
        n = f(&n, &x);
    }
    n
}
