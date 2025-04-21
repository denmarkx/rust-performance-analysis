// rand = "0.9"

use rand::prelude::*;
use std::env;

// for the use of cargo-single we have to traverse backwards.
include!{"../../../random_value.rs"}
include!{"../../../benchmark.rs"}
include!{"../../../util.rs"}

fn partition(array: &mut Vec<u32>, low: usize, high: usize) -> usize {
    let pivot = high;
    let mut i = low;

    for j in low..pivot-1 {
        unsafe {
            if array.get_unchecked(j) <= array.get_unchecked(pivot) {
                array.swap(j, i);
                i+=1;
            }
        }
    }
    array.swap(i, pivot);
    i
}

fn sort(array: &mut Vec<u32>, low: usize, high: usize) {
    if low >= high {
        return;
    }
    let pivot = partition(array, low, high);
    sort(array, low, pivot-1);
    sort(array, pivot+1, high);
}

fn t_quick_sort(array: Vec<Vec<u32>>) {
    for sub_array in array.iter() {
        benchmark(1, || sort(&mut sub_array.clone(), 1, array_size-1));
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
        let arr_data = read_from_file("../../test/quick_sort.txt");
        t_quick_sort(arr_data);
    }
    complete_benchmark();
}

