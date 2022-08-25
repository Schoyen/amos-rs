use num::complex::Complex;
use std::time::Instant;

use amos_rs::zbesi::{iv, iv_c};

fn profile_iv_evaluations() {
    println!("\n=====Profiling iv and iv_c evaluations=====");
    let now_iv = Instant::now();
    let foo_f = iv(1.2, Complex::new(1.0, 1.0), 1);

    for i in 0..10000000 {
        let foo = iv(1.2, Complex::new(1.0, 1.0), 1);
    }

    println!(
        "Total time iv:   {} ns",
        (now_iv.elapsed().as_nanos() as f64) / 1e9
    );

    let now_iv = Instant::now();
    let foo_f = iv_c(1.2, Complex::new(1.0, 1.0));

    for i in 0..10000000 {
        let foo = iv_c(1.2, Complex::new(1.0, 1.0));
    }

    println!(
        "Total time iv_c: {} ns",
        (now_iv.elapsed().as_nanos() as f64) / 1e9
    );
    println!("=====================================");
}

fn main() {
    profile_iv_evaluations();
}
