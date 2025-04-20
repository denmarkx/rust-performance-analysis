use std::time::Instant;
use std::time::Duration;
use lazy_static::lazy_static;
use std::sync::Mutex;

const AVERAGE_ONLY : bool = true;

/*
* Given a function and <n> for number of benchmarks,
* runs the given function n times and records the time
* it takes in milliseconds. Prints the elapsed time for
* each function and the total average time.
*/

// static mut TIMES: Vec<Duration> = Vec::new();
lazy_static! {
    static ref TIMES : Mutex<Vec<Duration>> = Mutex::new(Vec::new());
}


fn benchmark<F: FnMut() -> T, T>(n : usize, mut f: F) {
    // let mut t = Vec::with_capacity(n);

    // Iterate
    for i in 0..n {
        let now = Instant::now();
        f();
        TIMES.lock().unwrap().push(now.elapsed());
        if !AVERAGE_ONLY {
            println!("[{:?}] Time Elapsed: {:.2?}", i+1, now.elapsed());
        }
    }
}

fn complete_benchmark() {
    // Average time:
    let sum : Duration = TIMES.lock().unwrap().iter().sum();
    let count = TIMES.lock().unwrap().len() as u32;
    let average = sum / count;
    println!("Average Time: {:.2?}", average);
}
