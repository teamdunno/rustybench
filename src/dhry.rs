// based on original pascal version of dhrystone.

use std::time::{Duration, Instant};
use std::env::args;
use std::io::Write;

#[derive(Clone, Copy, PartialEq)]
enum Enumeration {
    Ident1,
    _Ident2,
    Ident3,
    _Ident4,
    _Ident5,
}

#[derive(Clone)]
struct RecordType {
    pointer_comp: Option<Box<RecordType>>,
    discr: Enumeration,
    enum_comp: Enumeration,
    int_comp: i32,
    _string_comp: String,
}

struct Dhrystone {
    int_glob: i32,
    bool_glob: bool,
    char1_glob: char,
    char2_glob: char,
    _array1_glob: [i32; 50],
    _array2_glob: [[i32; 50]; 50],
    pointer_glob: Option<Box<RecordType>>,
    _next_pointer_glob: Option<Box<RecordType>>,
}

impl Dhrystone {
    fn proc1(&mut self) {
        if let Some(ref mut ptr) = self.pointer_glob.take() {
    let mut _next = ptr.pointer_comp.take();
            ptr.pointer_comp = self.pointer_glob.clone();
            ptr.int_comp = 5;

            if let Some(ref mut next) = ptr.pointer_comp {
                next.int_comp = ptr.int_comp;
                self.proc3(&mut next.pointer_comp);
                if ptr.discr == Enumeration::Ident1 {
                    ptr.int_comp = 6;
                    self.proc6(ptr.enum_comp);
                }
            }
        }
    }

    fn proc2(&mut self, int_par_ref: &mut i32) -> bool {
        let mut int_loc = *int_par_ref + 10;
        if self.char1_glob == 'A' {
            int_loc -= 1;
            *int_par_ref = int_loc - self.int_glob;
        }
        true
    }

    fn proc3(&mut self, pointer_par_ref: &mut Option<Box<RecordType>>) {
        if let Some(ref mut ptr) = self.pointer_glob.take() {
            *pointer_par_ref = ptr.pointer_comp.clone();
            self.proc7(10, self.int_glob, &mut ptr.int_comp);
        }
    }

    fn proc4(&mut self) {
        self.bool_glob = self.char1_glob == 'A';
        self.char2_glob = 'B';
    }

    fn proc5(&mut self) {
        self.char1_glob = 'A';
        self.bool_glob = false;
    }

    fn proc6(&mut self, enum_par_val: Enumeration) {
        match enum_par_val {
            Enumeration::Ident1 => {},
            Enumeration::_Ident2 => {
                if self.int_glob > 100 {
                    self.bool_glob = true;
                }
            }
            Enumeration::Ident3 => {},
            _ => {},
        }
    }

    fn proc7(&mut self, int1_par_val: i32, int2_par_val: i32, int_par_ref: &mut i32) {
        let int_loc = int1_par_val + 2;
        *int_par_ref = int2_par_val + int_loc;
    }
}

pub fn dhry(secs: i128) -> i128 {
    let mut dhrystone = Dhrystone {
        int_glob: 0,
        bool_glob: false,
        char1_glob: ' ' ,
        char2_glob: ' ',
        _array1_glob: [0; 50],
        _array2_glob: [[0; 50]; 50],
        pointer_glob: Some(Box::new(RecordType {
            pointer_comp: None,
            discr: Enumeration::Ident1,
            enum_comp: Enumeration::Ident3,
            int_comp: 40,
            _string_comp: "DHRYSTONE PROGRAM, SOME STRING".to_string(),
        })),
        _next_pointer_glob: Some(Box::new(RecordType {
            pointer_comp: None,
            discr: Enumeration::Ident1,
            enum_comp: Enumeration::Ident3,
            int_comp: 40,
            _string_comp: "DHRYSTONE PROGRAM, SOME STRING".to_string(),
        })),
    };
    let start = Instant::now();
    let duration = Duration::from_secs(secs as u64);
    let mut runs: i128 = 0;
    while !(start.elapsed() >= duration) {
        runs += 1; // important NOTE: when compiling as debug, results WILL NOT be comparable with
                   // the original UnixBench
        dhrystone.proc5();
        dhrystone.proc4();
        let mut int1_loc = 2;
        let mut int2_loc = 3;
        let mut int3_loc = 4;
        let _bool_glob = !dhrystone.proc2(&mut int1_loc);
        while int1_loc < int2_loc {
            int3_loc = 5 * int1_loc - int2_loc;
            dhrystone.proc7(int1_loc, int2_loc, &mut int3_loc);
            int1_loc += 1;
        }
        dhrystone.proc1();
        int2_loc = int2_loc * int1_loc;
        int1_loc = int2_loc / int3_loc;
        let _ = int1_loc + int2_loc + int3_loc; // to make rustc shut up about unused assignments
    }

    runs / 10
}

#[allow(dead_code)]
fn main() {
    let mut arguments = vec![];
    let _ = args().into_iter().for_each(|value|arguments.push(value));
    if !arguments[arguments.len() - 1 as u64 as usize].parse::<u64>().is_ok() {
        eprintln!("Usage: dhry DURATION");
        std::process::exit(1);
    }
    let duration: i128 = arguments[arguments.len() - 1 as i128 as usize].parse::<i128>().unwrap();
    let runs: i128 = dhry(duration);
    eprint!("COUNT|{}|1|lps\n", runs); // print to stderr for compatibility
    std::io::stderr().flush().unwrap();
}

pub fn rustybench() -> i128 {
    if cfg!(debug_assertions) {
        dhry(1.into())
    } else {
        dhry(10.into()) // default val from ubench
    }
}
