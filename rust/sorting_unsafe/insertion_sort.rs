// rand = "0.9"
// lazy_static = "1.5.0"

use std::env;
use rand::prelude::*;

// for the use of cargo-single we have to traverse backwards.
include!{"../../../random_value.rs"}
include!{"../../../benchmark.rs"}
include!{"../../../util.rs"}

fn insertion_sort(array: &mut [u32]) {
    // Start with 2nd item:
    unsafe {
        for i in 1..array.len() {
            let n = *array.get_unchecked(i);
            let mut j = i;

            // Check previous:
            while j > 0 && *array.get_unchecked(j-1) > n {
                array[j] = *array.get_unchecked(j-1);
                j-=1;
            }

            // Insert front:
            array[j] = n;
        }
    }
}

fn t_insertion_sort(array: Vec<Vec<u32>>) {
    for sub_array in array.iter() {
        benchmark(1, || insertion_sort(&mut sub_array.clone()));
    }
}

fn main() {
    let mut args : Vec<String> = env::args().collect();
    warn_arguments(&mut args);
    let want_random : bool = use_random_values(&args[1]);

    if want_random {
        let array_size : usize = args[1].parse::<usize>().unwrap();
        let arr_data = randomize_array_set(1, args[2].parse::<u32>().unwrap(), array_size);
        t_insertion_sort(arr_data);
    } else {
        let arr_data = read_from_file("../../test/insertion_sort.txt");
        t_insertion_sort(arr_data);
    }
    complete_benchmark();
}
