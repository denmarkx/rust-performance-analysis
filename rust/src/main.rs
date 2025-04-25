// SORTING:
mod sorting;
use sorting::{bubble_sort, quick_sort, insertion_sort};

// MATH:
mod math;
use math::{matrix_mult};

// UTIL:
mod random_value;
mod benchmark;

// STD:
use std::env;

fn main() {

    let array = random_value::randomize_array_set(1, 5, 6);
    insertion_sort::start_sort(array);
}
