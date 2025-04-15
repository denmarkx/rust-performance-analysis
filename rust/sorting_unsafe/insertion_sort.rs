// rand = "0.9"

use rand::prelude::*;

// for the use of cargo-single we have to traverse backwards.
include!{"../../random_value.rs"}


fn main() {
    const ARRAY_SIZE : i32 = 5;
    let mut array = randomize_array(1, 5, ARRAY_SIZE);

    // Start with 2nd item:
    for i in 1..array.len() {
        let n = unsafe { *array.get_unchecked(i) };
        let mut j = i;

        // Check previous:
        while j > 0 && unsafe { *array.get_unchecked(j-1) } > n {
            array[j] = unsafe { *array.get_unchecked(j-1) };
            dbg!(j);
            j-=1;
        }

        // Insert front:
        array[j] = n;
    }

    dbg!(array);
}
