use crate::bindings::zbesh_;
use num::complex::Complex;
use std::os::raw::c_int;

pub fn zbesh(
    nu: f64,
    z: num::complex::Complex<f64>,
    kode: i32,
    m: i32,
    n: i32,
) -> Vec<Complex<f64>> {
    if m < 1 || m > 2 {
        panic!("m must 1 (hankel1) or 2 (hankel2)");
    }

    if kode < 1 || kode > 2 {
        panic!(
            "{}",
            format!("kode must be 1 (hankel{}) or 2 (hankel{}e)", m, m)
        );
    }

    if n < 1 {
        panic!("n must be at least 1");
    }

    let sign = nu.signum();
    let nu = nu * sign;

    // Sanity check that should be removed
    assert!(nu >= 0.0);

    let mut cyr = vec![0.0; n as usize];
    let mut cyi = vec![0.0; n as usize];
    let mut nz: c_int = 0;
    let mut ierr: c_int = 0;

    unsafe {
        zbesh_(
            &z.re,
            &z.im,
            &nu,
            &kode as *const c_int,
            &m as *const c_int,
            &n as *const c_int,
            cyr.as_mut_ptr(),
            cyi.as_mut_ptr(),
            &mut nz as *mut c_int,
            &mut ierr as *mut c_int,
        );
    }

    // Handle ierr and nz

    let mut cy: Vec<Complex<f64>> = cyr
        .iter()
        .zip(cyi.iter())
        .map(|(&re, &im)| Complex::new(re, im))
        .collect();

    // See amos/zbesh.f lines 89-95 on how to handle negative orders of nu
    if sign < 0.0 {
        let e_sign = match m {
            1 => 1.0,
            2 => -1.0,
            _ => panic!("Whoops! This case should not occur."),
        };

        for i in 0..(n as usize) {
            cy[i] =
                cy[i] * Complex::new(0.0, e_sign * std::f64::consts::PI * (nu + (i as f64))).exp();
        }
    }

    cy
}

pub fn hankel1(nu: f64, z: Complex<f64>, n: i32) -> Vec<Complex<f64>> {
    let kode: i32 = 1;
    let m: i32 = 1;

    zbesh(nu, z, kode, m, n)
}

pub fn hankel1e(nu: f64, z: Complex<f64>, n: i32) -> Vec<Complex<f64>> {
    let kode: i32 = 2;
    let m: i32 = 1;

    zbesh(nu, z, kode, m, n)
}

pub fn hankel2(nu: f64, z: Complex<f64>, n: i32) -> Vec<Complex<f64>> {
    let kode: i32 = 1;
    let m: i32 = 2;

    zbesh(nu, z, kode, m, n)
}

pub fn hankel2e(nu: f64, z: Complex<f64>, n: i32) -> Vec<Complex<f64>> {
    let kode: i32 = 2;
    let m: i32 = 2;

    zbesh(nu, z, kode, m, n)
}
