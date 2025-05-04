use std::time::Instant;
use std::time::Duration;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::{io, error};
use csv::Writer;
use std::fs::File;

const AVERAGE_ONLY : bool = false;

lazy_static! {
    static ref TIMES : Mutex<Vec<Duration>> = Mutex::new(Vec::new());
}

/*
* Given a function and <n> for number of benchmarks,
* runs the given function n times and records the time
* it takes in milliseconds. Prints the elapsed time for
* each function and the total average time.
*/
pub fn benchmark<F: FnMut() -> T, T>(n : usize, mut f: F) {
    // let mut t = Vec::with_capacity(n);

    // Iterate
    for i in 0..n {
        let now = Instant::now();
        f();
        TIMES.lock().unwrap().push(now.elapsed());
        if !AVERAGE_ONLY {
            println!("[{:?}] Time Elapsed: {:.2?}", TIMES.lock().unwrap().len(), now.elapsed());
        }
    }
}

pub fn complete_benchmark(algorithm : &str, unsafe_type : &str) {
    // Average time:
    let sum : Duration = TIMES.lock().unwrap().iter().sum();
    let count = TIMES.lock().unwrap().len() as u32;
    let average = sum / count;
    println!("Average Time: {:.2?}", average);
    write(algorithm, unsafe_type);
}

fn write(algorithm: &str, unsafe_type: &str) -> Result<(), Box<dyn error::Error>> {
    let mut wtr = csv::Writer::from_path(algorithm.to_owned() + ".csv")?;
    if !unsafe_type.is_empty() {
        let wrt = csv::Writer::from_path(algorithm.to_owned() + "_" + &unsafe_type.to_uppercase() + "_UNSAFE.csv")?;
    }
    wtr.write_record(&["TIME_MCS", "TIME_NS", "TIME_MS", "TIME_S"])?;

    // unwrap from mutex and iter the Vec<Duration>
    // row # is technically the iteration, so we won't enumerate.
    for t in TIMES.lock().unwrap().iter() {
        let micro : u128 = t.as_micros();
        let nano : u128 = t.as_nanos();
        let millis : u128 = t.as_millis();
        let seconds : f64 = t.as_secs_f64();
        wtr.write_record(&[micro.to_string(), nano.to_string(), millis.to_string(), seconds.to_string()])?;
    }
    wtr.flush();
    Ok(())
}
