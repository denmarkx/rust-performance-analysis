// rand = "0.9"
// lazy_static = "1.5.0"

use rand::prelude::*;
use std::env;

// for the use of cargo-single we have to traverse backwards.
include!{"../../../random_value.rs"}
include!{"../../../benchmark.rs"}
include!{"../../../util.rs"}

#[inline(never)]
fn bubble_sort(array: &mut[u32]) {
    // Bubble Sort Implementation:
    let mut i = 0;
    let mut j = 0;
    while i <= array.len() {
        while j <= array.len()-1 {
            if array[j] > array[j+1] {
                // swap:
                array.swap(j, j+1);
            }
            j += 1;
        }
        i += 1;
    }
}

fn t_bubble_sort(array: Vec<Vec<u32>>) {
    for sub_array in array.iter() {
        benchmark(1, || bubble_sort(&mut sub_array.clone()));
    }
}

fn main() {
    let mut args : Vec<String> = env::args().collect();
    warn_arguments(&mut args);
    let want_random : bool = use_random_values(&args[1]);

    if want_random {
        let array_size : usize = args[1].parse::<usize>().unwrap();
        let arr_data = randomize_array_set(1, args[2].parse::<u32>().unwrap(), array_size);
        t_bubble_sort(arr_data);
    } else {
        let arr_data = read_from_file("../../test/bubble_sort.txt");
        t_bubble_sort(arr_data);
    }
    complete_benchmark();
}

