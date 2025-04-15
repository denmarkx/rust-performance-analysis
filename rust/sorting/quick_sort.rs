// rand = "0.9"

use rand::prelude::*;

// for the use of cargo-single we have to traverse backwards.
include!{"../../random_value.rs"}

fn partition(array: &mut[u32]) -> usize {
    let pivot = array.len()-1;
    let mut i = 0;

    for j in 0..pivot {
        if array[j] <= array[pivot] {
            array.swap(j, i);
            i+=1;
        }
    }
    array.swap(i, pivot);
    i
}

fn sort(array: &mut[u32]) {
    if array.len() <= 1 {
        return;
    }
    let pivot = partition(array);
    sort(&mut array[..pivot]);
    sort(&mut array[pivot+1..]);
}

fn main() {
    const ARRAY_SIZE : i32 = 5;
    let mut array = randomize_array(1, 5, ARRAY_SIZE);

    sort(&mut array[..]);
    dbg!(array);
}
