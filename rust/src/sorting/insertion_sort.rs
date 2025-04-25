// rand = "0.9"

use rand::prelude::*;
use std::env;

// for the use of cargo-single we have to traverse backwards.
include!{"../../../random_value.rs"}
include!{"../../../benchmark.rs"}
include!{"../../../util.rs"}

fn insertion_sort(array: &mut [u32]) {
    // Start with 2nd item:
    for i in 1..array.len() {
        let n = array[i];
        let mut j = i;

        // Check previous:
        while j > 0 && array[j-1] > n {
            array[j] = array[j-1];
            j-=1;
        }

        // Insert front:
        array[j] = n;
    }
}

fn main() {
    let mut args : Vec<String> = env::args().collect();
    warn_arguments(&mut args);

    let array_size : usize = args[1].parse::<usize>().unwrap();
    let mut array = randomize_array(1, args[2].parse::<u32>().unwrap(), array_size);

    benchmark(args[3].parse::<usize>().unwrap(), || insertion_sort(&mut array));
}
