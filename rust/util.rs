/*
* If no arguments are supplied, it'll write them in-place.
* ..otherwise, stops and warns if arguments are illegal.
*/

fn warn_arguments(args : &mut Vec<String>) {
    if args.len() == 1 {
        args.push("5".to_string()); // Randomization upper bound.
        args.push("5".to_string()); // Iteration
        args.push("1".to_string()); // Benchmark once.
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
