use std::time::Instant;
use std::time::Duration;

const AVERAGE_ONLY : bool = true;

/*
* Given a function and <n> for number of benchmarks,
* runs the given function n times and records the time
* it takes in milliseconds. Prints the elapsed time for
* each function and the total average time.
*/
fn benchmark<F: FnMut() -> T, T>(n : usize, mut f: F) {
    let mut t = Vec::with_capacity(n);

    // Iterate
    for i in 0..n {
        let now = Instant::now();
        f();
        t.push(now.elapsed()*1000);
        if !AVERAGE_ONLY {
            println!("[{:?}] Time Elapsed: {:.2?}", i+1, now.elapsed()*1000);
        }
    }

    // Average time:
    let sum : Duration = t.iter().sum();
    let count = t.len() as u32;
    let average = sum / count;
    println!("Average Time: {:.2?}", average);
}
