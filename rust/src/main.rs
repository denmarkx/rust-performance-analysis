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
use std::fs::read_to_string;
use std::path::Path;

// Argument Parsing:
use argh::FromArgs;

#[derive(FromArgs)]
/// 
struct Args {
    /// file: if specified, all other arguments are useless.
    #[argh(option, short = 'f')]
    file: Option<String>,

    /// random range minimum
    #[argh(option, default = "1")]
    r_min: u32,

    /// random range maximum
    #[argh(option, default = "10")]
    r_max : u32,

    /// number of iterations:
    #[argh(option, default = "10")]
    n_iter : usize,
}

fn main() {
    let args : Args = argh::from_env();
    let mut array = vec![];

    // Check if we're from a file.
    if args.file.is_none() {
        // getting random arrays...
        array = random_value::randomize_array_set(args.r_min, args.r_max, args.n_iter);
    } else {
        // We can ensure that we're given a file (literally we did a check before this)
        // so I'll unwrap Option<> from arg.sfile.
        let file_name: String = args.file.unwrap();

        // Let's check if this file is actually real first.
        if Path::new(&file_name).exists() {
            array = read_from_file(&file_name);
        } else {
            panic!("Input file: '{}' does not exist.", file_name);
        }
    }

    // Do the benchmark:
    insertion_sort::do_benchmark(array, true);
}

/*
* Converts everything within a given file to a 2D Vector.
* Line format of file:
* <array size> <element> <element>
*/
fn read_from_file(filename: &str) -> Vec<Vec<u32>> {
    // our vector:
    let mut array : Vec<Vec<u32>> = vec![];

    // actual data:
    for line in read_to_string(filename).unwrap().lines() {
        let split_string = line.split(" ");
        let mut child_vec: Vec<u32> = vec![];
        let mut i = 0;

        for part in split_string {
            // first of part is our array size.
            if i == 0 {
                i += 1;
                continue;
            }
            child_vec.push(part.parse::<u32>().unwrap());
            i += 1;
        }
        array.push(child_vec.clone());
    }
    array
}
