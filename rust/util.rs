/*
* If no arguments are supplied, it'll write them in-place.
* ..otherwise, stops and warns if arguments are illegal.
*/

use std::fs::read_to_string;

fn use_random_values(arg_1: &str) -> bool {
    return arg_1 != "-f";
}

fn warn_arguments(args : &mut Vec<String>) {
    if args.len() == 2 {
        if use_random_values(&args[1]) {
            args.push("5".to_string()); // Randomization upper bound.
            args.push("5".to_string()); // Iteration
            return;
        }
        return;
    }

    // Default benchmark count if not given.
    if args.len() == 3 {
        args.push("1".to_string());
        return;
    }

    // Illegal operations:
    if args.len() == 2 {
        panic!("Incorrect usage. Valid parameters: <random_value> <iteration_count>");
    }

    if args.len() >= 5 {
        panic!("Incorrect usage. Valid parameters: <random_value> <iteration_count> <benchmark_count>.");
    }
}

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
