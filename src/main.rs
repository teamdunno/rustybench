use std::env;
mod dhry;
mod whets;
mod shared;
mod arith;

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let mut args = vec![];
    println!("RustyBench - version {}", version);

    let _ = env::args().into_iter().for_each(|value|args.push(value));

    if args.contains(&String::from("whets")) {
        whets::whets();
    }
    if args.contains(&String::from("dhry")) {
        dhry::dhry();
    }

    if args.contains(&String::from("arithoh")) {
        arith::arithoh(args);
    }
}
