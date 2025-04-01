use std::sync::{OnceLock, Mutex};
use std::time::SystemTime;
use std::env::var;

pub fn dtime() -> f64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs_f64()
}

pub fn pout(title: &str, ops: f64, type_: i32, checknum: f64, time: f64, calibrate: i32, section: usize) { // TODO: this is a mess :P
    static CHECK: OnceLock<Mutex<f64>> = OnceLock::new();
    static LOOP_TIME: OnceLock<Mutex<[f64; 10]>> = OnceLock::new();
    // static HEADINGS: OnceLock<Mutex<[String; 10]>> = OnceLock::new();
    static HEADINGS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    static RESULTS: OnceLock<Mutex<[f64; 10]>> = OnceLock::new();
    static LOOP_MOPS: OnceLock<Mutex<[f64; 10]>> = OnceLock::new();
    static LOOP_MFLOPS: OnceLock<Mutex<[f64; 10]>> = OnceLock::new();
    static TIME_USED: OnceLock<Mutex<f64>> = OnceLock::new();
    let mut check = CHECK.get_or_init(|| Mutex::new(0.0)).lock().unwrap();
    let mut loop_time = LOOP_TIME.get_or_init(|| Mutex::new([0.0; 10])).lock().unwrap();
    let mut headings = HEADINGS.get_or_init(|| Mutex::new(vec![String::from(""); 10])).lock().unwrap();
    let mut results = RESULTS.get_or_init(|| Mutex::new([0.0; 10])).lock().unwrap();
    let mut loop_mops = LOOP_MOPS.get_or_init(|| Mutex::new([0.0; 10])).lock().unwrap();
    let mut loop_mflops = LOOP_MFLOPS.get_or_init(|| Mutex::new([0.0; 10])).lock().unwrap();
    let mut time_used = TIME_USED.get_or_init(|| Mutex::new(0.0)).lock().unwrap();
    *check += checknum;
    loop_time[section] = time;
    headings[section] = String::from(title);
    *time_used += time;
    // print!("{:18} {:24.17}    ", headings[section], results[section]);
    if type_ == 1 {
        let mflops = if time > 0.0 { ops / (1_000_000.0 * time) } else { 0.0 };
        println!("ops: {}, time: {}", ops, time);
        loop_mops[section] = 99999.0;
        loop_mflops[section] = mflops;
        // println!(" {:9.3}          {:9.3}", loop_mflops[section], loop_time[section]);
    } else {
        let mops = if time > 0.0 { ops / (1_000_000.0 * time) } else { 0.0 };
        println!("{}", ops);
        loop_mops[section] = mops;
        loop_mflops[section] = 0.0;
        // println!("           {:9.3}{:9.3}", loop_mops[section], loop_time[section]);
    }
}

pub fn running_on_rustybench() -> bool {
    match var("RUSTYBENCH") {
        Ok(_val) => true,
        Err(_e) => false,
    }
}
