use crate::bindings::{zbesi_, zbesk_};
use num;
use std::os::raw::c_int;

pub fn zbesi(
    nu: f64,
    z: num::complex::Complex<f64>,
    kode: i32,
    n: i32,
) -> Vec<num::complex::Complex<f64>> {
    if kode < 1 || kode > 2 {
        panic!("kode must be 1 (iv) or 2 (ive)");
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
        zbesi_(
            &z.re,
            &z.im,
            &nu,
            &kode as *const c_int,
            &n as *const c_int,
            cyr.as_mut_ptr(),
            cyi.as_mut_ptr(),
            &mut nz as *mut c_int,
            &mut ierr as *mut c_int,
        );
    }

    // Handle ierr and nz

    // See amos/zbesi.f lines 78-90 on how to handle negative orders of nu
    if sign < 0.0 {
        let mut cy_kr = vec![0.0; n as usize];
        let mut cy_ki = vec![0.0; n as usize];
        let mut nz: c_int = 0;
        let mut ierr: c_int = 0;

        unsafe {
            zbesk_(
                &z.re,
                &z.im,
                &nu,
                &kode as *const c_int,
                &n as *const c_int,
                cy_kr.as_mut_ptr(),
                cy_ki.as_mut_ptr(),
                &mut nz as *mut c_int,
                &mut ierr as *mut c_int,
            );
        }

        // Handle nz and ierr

        // Handle lines 72-78 in zbesi.f
        for i in 0..(n as usize) {
            let arg = std::f64::consts::PI * (nu + (i as f64));
            cyr[i] = cyr[i] + (2.0 / std::f64::consts::PI) * arg.sin() * cy_kr[i];
            cyi[i] = cyi[i] + (2.0 / std::f64::consts::PI) * arg.sin() * cy_ki[i];
        }
    }

    cyr.iter()
        .zip(cyi.iter())
        .map(|(&re, &im)| num::complex::Complex::new(re, im))
        .collect()
}

pub fn iv(nu: f64, z: num::complex::Complex<f64>, n: i32) -> Vec<num::complex::Complex<f64>> {
    let kode: i32 = 1;

    zbesi(nu, z, kode, n)
}

pub fn ive(nu: f64, z: num::complex::Complex<f64>, n: i32) -> Vec<num::complex::Complex<f64>> {
    let kode: i32 = 2;

    zbesi(nu, z, kode, n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_zbesi() {
        let kode: i32 = 1;
        let n: i32 = 3;

        let foo = zbesi(0.0, num::complex::Complex::new(1.0, 1.0), kode, n);
        assert_eq!(foo.len(), n as usize);
    }

    #[test]
    fn test_iv() {
        let n: i32 = 2;

        let foo = iv(-1.0, num::complex::Complex::new(1.0, 1.0), n);
        assert_eq!(foo.len(), n as usize);
        dbg!(foo);
        let foo_2 = iv(1.0, num::complex::Complex::new(1.0, 1.0), n);
        // Should be the same as foo
        dbg!(foo_2);
    }

    #[test]
    fn test_ive() {
        let n: i32 = 1;

        let foo = ive(-1.0, num::complex::Complex::new(1.0, 1.0), n);
        dbg!(&foo);
        assert_eq!(foo.len(), n as usize);
    }
}
