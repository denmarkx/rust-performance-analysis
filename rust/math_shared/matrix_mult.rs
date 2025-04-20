// rand = "0.9"

use rand::prelude::*;
use std::env;

// for the use of cargo-single we have to traverse backwards.
include!{"../../../random_value.rs"}
include!{"../../../benchmark.rs"}
include!{"../../../util.rs"}

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

fn main() {
    let mut args : Vec<String> = env::args().collect();
    warn_arguments(&mut args);

    let array_size : usize = args[3].parse::<usize>().unwrap();
    let mut mat_a = randomize_array_set(
        args[1].parse::<u32>().unwrap(),
        args[2].parse::<u32>().unwrap(),
        array_size
    );

    let mut mat_b = randomize_array_set(
        args[1].parse::<u32>().unwrap(),
        args[2].parse::<u32>().unwrap(),
        array_size
    );

    benchmark(args[3].parse::<usize>().unwrap(), || matrix_mult(&mut mat_a, &mut mat_b));
}
