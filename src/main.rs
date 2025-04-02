use std::env;
mod dhry;
mod whets;
mod arith;

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let mut args = vec![];
    println!("RustyBench - version {}", version);
    let _ = env::args().into_iter().for_each(|value|args.push(value));
    if args.contains(&String::from("arithoh")) {
        let runs: i128 = arith::rustybench();
        println!("TODO!: runs: {}", runs)
    }
/*
    if args.contains(&String::from("whets")) {
        whets::whets();
    } */
    if args.contains(&String::from("dhry")) {
        let runs: i128 = dhry::rustybench();
        println!("TODO: runs: {}", runs)
    }
/*
    if args.contains(&String::from("arithoh")) {
        arith::arithoh(args);
    } */
}
