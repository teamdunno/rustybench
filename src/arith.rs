use std::time::{Duration, Instant};

fn dumb_stuff(i: f64) -> f64 {
    let x: f64 = i.clone().into();
    let y: f64 = x * x;
    let mut z: f64 = 0.0;

    for _i in 2..=101 {
        z += y / (y - 1.0);
    }

    x + y + z
}

pub fn arithoh(args: Vec<String>) {
    if !args[args.len() - 1 as u64 as usize].parse::<u64>().is_ok() {
        panic!("Usage: arithoh DURATION")
    }
    let duration = Duration::new(args[args.len() - 1 as u64 as usize].parse::<u64>().unwrap(), 0);
    let mut iterations = 0;
    let start = Instant::now();
    let mut result = 0 as f64;

    while start.elapsed() < duration {
        iterations += 1;
        result = dumb_stuff(result);
    }
    println!("COUNT|{}|1|lps", iterations)
}
