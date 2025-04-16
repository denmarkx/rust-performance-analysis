// rand = "0.9"

use rand::prelude::*;
use std::env;

// for the use of cargo-single we have to traverse backwards.
include!{"../../random_value.rs"}
include!{"../../../benchmark.rs"}
include!{"../../../util.rs"}

fn partition(array: &mut Vec<u32>, low: usize, high: usize) -> usize {
    let pivot = high;
    let mut i = low;

    for j in low..pivot-1 {
        if array[j] <= array[pivot] {
            array.swap(j, i);
            i+=1;
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


fn main() {
    let mut args : Vec<String> = env::args().collect();
    warn_arguments(&mut args);

    let array_size : usize = args[1].parse::<usize>().unwrap();
    let mut array = randomize_array(1, args[2].parse::<u32>().unwrap(), array_size);

    benchmark(args[3].parse::<usize>().unwrap(), || sort(&mut array, 1, array_size-1));
}
