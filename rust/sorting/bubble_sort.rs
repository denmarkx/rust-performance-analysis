// rand = "0.9"

use rand::prelude::*;
use std::env;

// for the use of cargo-single we have to traverse backwards.
include!{"../../random_value.rs"}
include!{"../../../benchmark.rs"}
include!{"../../../util.rs"}

fn bubble_sort(array: &mut[u32]) {
    // Bubble Sort Implementation:
    for _i in 0..array.len() {
        for j in 0..array.len()-1 {
            if array[j] > array[j+1] {
                // swap:
                array.swap(j, j+1);
            }
        }
    }
}

fn main() {
    let mut args : Vec<String> = env::args().collect();
    warn_arguments(&mut args);

    let array_size : usize = args[1].parse::<usize>().unwrap();
    let mut array = randomize_array(1, args[2].parse::<u32>().unwrap(), array_size);

    benchmark(args[3].parse::<usize>().unwrap(), || bubble_sort(&mut array));
}
