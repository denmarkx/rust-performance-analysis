// rand = "0.9"

use std::time::Instant;
use rand::prelude::*;

// for the use of cargo-single we have to traverse backwards.
include!{"../../random_value.rs"}

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
    const ARRAY_SIZE : usize = 52025;
    let mut array = randomize_array(1, 5, ARRAY_SIZE);

    let now = Instant::now();

    sort(&mut array, 1, ARRAY_SIZE-1);

    let elapsed = now.elapsed();
    println!("Time Elapsed: {:.6?}", elapsed);
}
