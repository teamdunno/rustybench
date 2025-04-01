use std::time::SystemTime;
use std::vec::Vec;
use crate::shared;

fn whetstones(xtra: i32, x100: i32, surpress: bool, calibrate: i32) -> f64 {
    let mut n1 = i32::from(0);
    let mut n2 = i32::from(0);
    let mut n3 = i32::from(0);
    let mut n4 = i32::from(0);
    let mut n5 = i32::from(0);
    let mut n6 = i32::from(0);
    let mut n7 = i32::from(0);
    let mut n8 = i32::from(0);
    let mut i = i32::from(0);
    let mut ix = i32::from(0);
    let mut n1mult = i32::from(0);
    let mut j = i32::from(0);
    let mut k = i32::from(0);
    let mut l = i32::from(0);
    let mut x = f64::from(0.0);
    let mut y = f64::from(0.0);
    let mut z = f64::from(0.0);
    let mut e1: Vec<f64> = Vec::with_capacity(5);
    e1.append(&mut vec![0.0, 0.0, 0.0, 0.0, 0.0]);
    let mut t = f64::from(0.49999975);
    let mut t0 = t.clone();
    let mut t1 = f64::from(0.50000025);
    let mut t2 = f64::from(2.0);
    let mut check = f64::from(0.0);
    e1[4] = shared::dtime();
    let mut timea = e1[4].clone();
    let mut timeb = e1[4].clone();
    check = 0.0 as f64;
    n1 = 12*x100 as i32;
    n2 = 14*x100 as i32;
    n3 = 345*x100 as i32;
    n4 = 210*x100 as i32;
    n5 = 32*x100 as i32;
    n6 = 899*x100 as i32;
    n7 = 616*x100 as i32;
    n8 = 93*x100 as i32;
    n1mult = 10 as i32;

    /* Section 1, Array elements */
    e1[0] = 1.0;
    e1[1] = -1.0;
    e1[2] = -1.0;
    e1[3] = -1.0;

    for _ in 0..xtra {
        ix += 1;
        for _ in 0..(n1 * n1mult){
            i += 1;
            e1[0] = (e1[0] + e1[1] + e1[2] - e1[3]) * t;
            e1[1] = (e1[0] + e1[1] - e1[2] + e1[3]) * t;
            e1[2] = (e1[0] - e1[1] + e1[2] + e1[3]) * t;
            e1[3] = (-e1[0] + e1[1] + e1[2] + e1[3]) * t;
        }
        t = 1.0 - t;
    }
    t = t0;

    timeb = shared::dtime();
    let flops = (n1 * 16) as f64 * xtra as f64;
    if !surpress {
        shared::pout("N1 floating point\0", flops, 1, e1[3], timeb, calibrate, 1);
    }
    timeb - timea
}

pub fn whets() {
    let mut calibrate = 1 as i32;
    let mut count = 10 as i32;
    let mut xtra = 1 as i32;
    let mut time_to_be_used = 0.5;
    let running_from_rustybench = shared::running_on_rustybench();
    let x100 = 100 as i32;
    println!("##########################################");
    println!("RustyBench Whetstone benchmark");
    println!("Calibrate");
    if running_from_rustybench {
        time_to_be_used = 2.0 // TODO: add more, result file etc.
    }
    while count > 0 {
        let time_used: f64 = whetstones(xtra, x100, true, calibrate).into();

        println!("{:11.2} Seconds {:10} Passes (x 100)", time_used, xtra);

        calibrate += 1;
        count -= 1;

        if time_used > time_to_be_used {
            break;
        } else {
            xtra *= 5;
        }
    }
    whetstones(xtra * 5, x100, false, calibrate);
}
