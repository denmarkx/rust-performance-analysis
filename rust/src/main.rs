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
use std::process;

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
    r_max: u32,

    /// number of iterations:
    #[argh(option, default = "10", short = 'n')]
    n_iter: usize,

    /// algorithm to benchmark
    #[argh(option, short = 'a', from_str_fn(validate_algorithm))]
    algorithm: String,

    /// runs the unsafe version
    #[argh(option, default = "String::from(\"\")", short = 'u', from_str_fn(validate_unsafe_type))]
    unsafe_type : String,

    /// length of inner random array. if not specified or is 0, will be random.
    /// REQUIRED to be non-zero for matrix multiplication.
    /// for sorting algorithms, this will OVERRIDE n_iter.
    #[argh(option, default = "0", short = 'l')]
    inner_length : usize,
}

fn validate_algorithm(value: &str) -> Result<String, String> {
    // Panic if not apart of our main algorithms:
    match value {
        "insertion" => {},
        "bubble" => {},
        "quick" => {},
        "matrix" => {},
        &_ => { 
            eprintln!("Invalid argument specified! Use: [insertion, bubble, quick, matrix]");
            process::exit(1);
            },
    };
    Ok(String::from(value))
}

fn validate_unsafe_type(value: &str) -> Result<String, String> {
    match value {
        "oob" => {},
        "rptr" => {},
        &_ => {
            eprintln!("Invalid argument to --unsafe-type! Use: [oob, rptr]");
            process::exit(1);
            },
    };
    Ok(String::from(value))
}

fn main() {
    // args:
    let args : Args = argh::from_env();
    let algorithm: String = args.algorithm;

    // Arrays:
    let mut array: Vec<Vec<u32>> = vec![];
    let mut array2: Vec<Vec<u32>> = vec![];

    // Check if we're from a file.
    if args.file.is_none() {
        // Matrix Multiplication requires another array.
        array = random_value::randomize_array_set(args.r_min, args.r_max, args.n_iter, args.inner_length);

        if algorithm == "matrix" {
            // Let's make use args.inner_length is NOT 0.
            if args.inner_length == 0 {
                eprintln!("--inner-length argument must be non-zero and postive for matrix multiplication.");
                process::exit(1);
            }

            array = random_value::randomize_array_set(args.r_min, args.r_max, args.n_iter, args.inner_length);
            array2 = random_value::randomize_array_set(args.r_min, args.r_max, args.n_iter, args.inner_length);
        }

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


    // Seems like the only way to really do this would be to
    // actually place all your args in some DS and just allow
    // do_benchmark to accept that as your argument.
    //
    // I'll avoid that only because of matrix multipication expecting more.
    // algorithm is required so we can safely unwrap.
    match algorithm.as_str() {
        // SORTING:
        "insertion" => insertion_sort::do_benchmark(&mut array, &args.unsafe_type),
        "bubble" => bubble_sort::do_benchmark(&mut array, &args.unsafe_type),
        "quick" => quick_sort::do_benchmark(&mut array, &args.unsafe_type),

        // MATH:
        "matrix" => matrix_mult::do_benchmark(args.n_iter, &mut array, &mut array2, &args.unsafe_type),

        // UNIMPLEMENTED:
        &_ => todo!("algorithm: {}", algorithm),
    }

    // This spits out some useful info like avg. time and it
    // also writes to CSV.
    benchmark::complete_benchmark(&algorithm.as_str());
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
        array.push(child_vec);
    }
    array
}
