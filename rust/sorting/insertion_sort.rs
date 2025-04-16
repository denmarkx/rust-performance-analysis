// rand = "0.9"

use std::time::Instant;
use rand::prelude::*;

// for the use of cargo-single we have to traverse backwards.
include!{"../../random_value.rs"}

fn main() {
    const ARRAY_SIZE : usize = 5;
    let mut array = randomize_array(1, 5, ARRAY_SIZE);

    // Start with 2nd item:
    let now = Instant::now();
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
    let elapsed = now.elapsed();
    println!("Time Elapsed: {:.6?}", elapsed);
}
