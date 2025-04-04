use std::i128;
use std::io::Write;
use std::time::{Duration, Instant};
use std::env::args;

struct Arithoh {}

impl Arithoh {
    pub fn dumb_stuff(&mut self, i: f64) -> f64 {
        let x: f64 = i.clone().into();
        let y: f64 = x * x;
        let mut z: f64 = 0.0;
        for _i in 2..101 {
            z += y / (y - 1.0);
        }
        x + y + z
    }
}

pub fn arithoh(secs: i128) -> i128 {
    let mut result: f64 = 0.0;
    let mut arithoh = Arithoh {};
    let start = Instant::now();
    let duration = Duration::from_secs(secs as u64);
    let mut runs: i128 = 0;

    while !(start.elapsed() >= duration) {
        runs += 1;
        result = arithoh.dumb_stuff(result);
    }
    runs
}

#[allow(dead_code)]
fn main() {
    let mut arguments = vec![];
    let _ = args().into_iter().for_each(|value|arguments.push(value));
    if !arguments[arguments.len() - 1 as u64 as usize].parse::<u64>().is_ok() {
        eprintln!("Usage: arithoh DURATION");
        std::process::exit(1);
    }
    let duration: i128 = arguments[arguments.len() -1 as u64 as usize].parse::<i128>().unwrap();
    let runs: i128 = arithoh(duration);
    eprintln!("COUNT|{}|1|lps", runs); // print to stderr for compatibility
    std::io::stderr().flush().unwrap();
}

pub fn rustybench() -> i128 {
    if cfg!(debug_assertions) {
        arithoh(1.into())
    } else {
        arithoh(10.into()) // default val from ubench
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_arithoh() {
        let runs = arithoh(1.into());
        assert!(runs > 1);
    }
}
