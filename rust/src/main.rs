// SORTING:
mod sorting { pub mod bubble_sort; pub mod quick_sort; pub mod insertion_sort; }
use sorting::{bubble_sort, quick_sort, insertion_sort};

// MATH:
mod math { pub mod matrix_mult; }
use math::{matrix_mult};

// UTIL:
mod random_value;
mod benchmark;

// STD:
use std::env;

fn main() {

    let array = random_value::randomize_array_set(1, 5, 6);
    insertion_sort::do_benchmark(array, true);
}
