use std::{env, result};
use std::io::Write;
use std::thread;
use gethostname::gethostname;
use tabled::builder::Builder;
use tabled::settings::{Style, object::Columns, Alignment};
mod dhry;
mod whets;
mod arith;
mod index;

#[allow(non_upper_case_globals)]
static version: &str = env!("CARGO_PKG_VERSION");

struct BenchResults {
    benchmark: String,
    multi_result: i32,
    multi_index: Option<i32>,
    baseline: Option<i32>,
    result: i32,
    index: Option<i32>,
}

fn log_results(bench_results: Vec<BenchResults>) {
    let cpus = num_cpus::get() as i32;

    println!("=========================================");
    println!("RustyBench benchmarks - version {version}\n");
    println!("System: {}",
        gethostname().into_string().unwrap());
    println!("OS: {}",
        env::consts::OS);
    println!("Machine: {}",
        sysinfo::System::cpu_arch());

    println!("------------------------------------------");
    println!("Benchmark run: TODO, insert date/time here\nResults:\n");
    let mut builder = Builder::default();
    builder.push_record(["BENCHMARK", "CPUs", "BASELINE", "RESULT", "INDEX"]);
    for result in bench_results {
        builder.push_record([
            result.benchmark,
            1.to_string(),
            result.baseline.unwrap_or(0.into()).to_string(),
            result.result.to_string(),
            result.index.unwrap_or(0.into()).to_string()
        ]);
    }
    let mut table = builder.build();
    table.with(Style::empty());
    table.modify(Columns::first(), Alignment::right());
    println!("{}", table);
}

fn main() {
    println!("RustyBench - version {version}");
    if cfg!(debug_assertions) {
        println!(
            "WARNING! RustyBench compiled in debug mode!
             Some benchmarks might be ran for a shorter time (1 second instead of 10) for easier debugging.
             Resulting scores etc. won't be comparable!"
        );
    }
    let mut args = vec![];
    let _ = env::args().into_iter().for_each(|value|args.push(value));
    let mut arithoh = false;
    let mut dhry = false;
    let mut whets = false;
    let mut global_results: Vec<BenchResults> = vec![];
    let cpus = num_cpus::get() as i32;

    if args.contains(&String::from("arithoh")) {
        arithoh = true;
    }
    if args.contains(&String::from("dhry")) {
        dhry = true;
    }

    if arithoh {
        let mut results = vec![];
        let mut results_multi = vec![];
        print!("1 x Arithoh ");
        std::io::stdout().flush().unwrap();
        for i in 1..4 {
            print!(" {}", i);
            results.push(arith::rustybench() as i32);
            std::io::stdout().flush().unwrap();
        }
        print!("\n");
        print!("{} x Arithoh ", cpus);
        for i in 1..4 {
            print!(" {}", i);
            std::io::stdout().flush().unwrap();
            let mut result_: i32 = 0;
            let mut childs = vec![];
            for _i in 0..(cpus) {
                childs.push(thread::spawn(move || {
                    arith::rustybench()
                }))
            }
            for child in childs {
                let result = child.join().unwrap();
                result_ += result as i32;
            }
            results_multi.push(result_);
        }
        print!("\n");
        std::io::stdout().flush().unwrap();

        let final_results = BenchResults {
            result: results[0],
            multi_result: results_multi[0],
            benchmark: String::from("Arithoh"),
            baseline: None,
            index: None,
            multi_index: None,
        };
        global_results.push(final_results);
    }

    if dhry {
        let mut results = vec![];
        let mut results_multi = vec![];
        print!("1 x Dhrystone ");
        std::io::stdout().flush().unwrap();
        for i in 1..11 {
            print!(" {}", i);
            results.push(dhry::rustybench() as i32);
            std::io::stdout().flush().unwrap();
        }
        print!("\n");
        print!("{} x Dhrystone ", cpus);
        for i in 1..11 {
            print!(" {}", i);
            std::io::stdout().flush().unwrap();
            let mut result_: i32 = 0;
            let mut childs = vec![];
            for _i in 0..(cpus) {
                childs.push(thread::spawn(move || {
                    dhry::rustybench()
                }))
            }
            for child in childs {
                let result = child.join().unwrap();
                result_ += result as i32;
            }
            results_multi.push(result_);
        }
        print!("\n");
        std::io::stdout().flush().unwrap();

        let indexed = *index::get_indexed_scores().get("dhry").unwrap() as i32;
        let scores = index::get_score(results.clone(), indexed);
        let scores_multi = index::get_score(results_multi.clone(), indexed);
        // println!("Single-Core score: {}", scores.get("final").unwrap()[0]);
        let final_results = BenchResults {
            result: results[0], // TODO
            multi_result: results_multi[0], // TODO
            index: Some(scores.get("final").unwrap()[0] as i32),
            multi_index: Some(scores_multi.get("final").unwrap()[0] as i32),
            baseline: Some(indexed),
            benchmark: String::from("Dhrystone")
        };
        global_results.push(final_results);
        // println!("Multi-Core score: {}", scores_multi.get("final").unwrap()[0])
    }
    log_results(global_results);
}
