// SORTING:
mod sorting;
use sorting::{bubble_sort};

// MATH:

// UTIL:
mod random_value;
mod benchmark;

// STD:
use std::env;

fn main() {

    let array = random_value::randomize_array_set(1, 5, 6);
    bubble_sort::do_sort(array);
}
