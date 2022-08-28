use crate::bindings::{zbesi_, zbesk_};
use num::complex::Complex;
use std::fmt;
use std::os::raw::{c_double, c_int};

enum ZbesiErrors<T> {
    // Definitions of the errors are found in amos/zbesi.f lines 42-60
    Underflow { res: T, nz: i32 }, // NZ > 0
    InputError,                    // IERR=1
    Overflow,                      // IERR=2
    LossOfSignificance(T),         // IERR=3
    CompleteLossOfSignificance,    // IERR=4
    TerminationConditionNotMet,    // IERR=5
}

impl<T> fmt::Display for ZbesiErrors<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZbesiErrors::Underflow { res, nz } => {
                write!(f, "Underflow, number of components set to zero: {}", nz)
            }
            ZbesiErrors::InputError => write!(f, "Input error (IERR=1)"),
            ZbesiErrors::Overflow => write!(f, "Overflow, no computation done (IERR=2)"),
            ZbesiErrors::LossOfSignificance(res) => write!(
                f,
                "|z| or nu large, computation done, but loss of significance (IERR=3)"
            ),
            ZbesiErrors::CompleteLossOfSignificance => {
                write!(f, "|z| or nu too large, no computation done (IERR=4)")
            }
            ZbesiErrors::TerminationConditionNotMet => write!(
                f,
                "Algorithm termination condition not met, no computation done (IERR=5)"
            ),
        }
    }
}

pub fn zbesi(nu: f64, z: Complex<f64>, kode: i32) -> Complex<f64> {
    if kode < 1 || kode > 2 {
        panic!("kode must be 1 (iv) or 2 (ive)");
    }

    let n = 1;
    let sign = nu.signum();
    let nu = nu * sign;

    // Sanity check that should be removed
    assert!(nu >= 0.0);

    let mut cyr = 0.0;
    let mut cyi = 0.0;
    let mut nz: c_int = 0;
    let mut ierr: c_int = 0;

    unsafe {
        zbesi_(
            &z.re,
            &z.im,
            &nu,
            &kode as *const c_int,
            &n as *const c_int,
            &mut cyr as *mut c_double,
            &mut cyi as *mut c_double,
            &mut nz as *mut c_int,
            &mut ierr as *mut c_int,
        );
    }

    // Handle ierr and nz

    let mut cy = Complex::new(cyr, cyi);

    // See amos/zbesi.f lines 78-90 on how to handle negative orders of nu
    // Also, since sin(pi * nu) = 0 when nu is an integer, we test nu.floor() == nu to avoid
    // computing kv unnecessarily.
    if sign < 0.0 && nu.floor() != nu {
        let mut cy_kr = 0.0;
        let mut cy_ki = 0.0;
        let mut nz: c_int = 0;
        let mut ierr: c_int = 0;

        unsafe {
            zbesk_(
                &z.re,
                &z.im,
                &nu,
                &kode as *const c_int,
                &n as *const c_int,
                &mut cy_kr as *mut c_double,
                &mut cy_ki as *mut c_double,
                &mut nz as *mut c_int,
                &mut ierr as *mut c_int,
            );
        }

        // Handle nz and ierr

        let cy_k = Complex::new(cy_kr, cy_ki);

        // In the case where kode == 2, i.e., we compute the exponentially scaled Bessel functions
        // ive and kve, we need to handle that the scaling is different for iv and kv.
        // That is,
        //
        //      ive(nu, z) = exp(-abs(z.re)) * iv(nu, z)    (see line 19 in zbesi.f),
        //
        //  and
        //
        //      kve(nu, z) = exp(z) * kv(nu, z)             (see line 21 in zbesk.f).
        //
        //  This means that for -nu < 0 with exponentially scaling we get
        //
        //      ive(-nu, z) = exp(-abs(z.re)) * iv(-nu, z)
        //                  = exp(-abs(z.re)) * (
        //                      iv(nu, z) + (2 / pi) * sin(pi * nu) kv(nu, z)
        //                  )
        //                  = ive(nu, z)
        //                      + (2 / pi) * sin(pi * nu) * exp(-abs(z.re)) * kv(nu, z)
        //                  = ive(nu, z) + (2 / pi) * sin(pi * nu)
        //                      * exp(-abs(z.re)) * exp(-z) * exp(z) * kv(nu, z)
        //                  = ive(nu, z) + (2 / pi) * sin(pi * nu)
        //                      * exp(-(z.re + abs(z.re)) - 1j * z.im) * kve(nu, z)

        let mut k_scaling = Complex::new(1.0, 0.0);

        if kode == 2 {
            k_scaling = (-z.re.abs() - z).exp();
        }

        // Handle lines 72-78 in zbesi.f
        let sin_nupi = (std::f64::consts::PI * nu).sin();
        cy = cy + (2.0 / std::f64::consts::PI) * sin_nupi * k_scaling * cy_k;
    }

    cy
}

pub fn iv(nu: f64, z: Complex<f64>) -> Complex<f64> {
    let kode: i32 = 1;

    zbesi(nu, z, kode)
}

pub fn ive(nu: f64, z: Complex<f64>) -> Complex<f64> {
    let kode: i32 = 2;

    zbesi(nu, z, kode)
}

pub fn iv_real(nu: f64, z: f64) -> f64 {
    let kode: i32 = 1;

    zbesi(nu, Complex::new(z, 0.0), kode).re
}

pub fn ive_real(nu: f64, z: f64) -> f64 {
    let kode: i32 = 2;

    zbesi(nu, Complex::new(z, 0.0), kode).re
}

pub fn zbesi_vec(nu: f64, z: Complex<f64>, kode: i32, n: i32) -> Vec<Complex<f64>> {
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

    let mut cy: Vec<Complex<f64>> = cyr
        .iter()
        .zip(cyi.iter())
        .map(|(&re, &im)| Complex::new(re, im))
        .collect();

    // See amos/zbesi.f lines 78-90 on how to handle negative orders of nu
    // Also, since sin(pi * nu) = 0 when nu is an integer, we test nu.floor() == nu to avoid
    // computing kv unnecessarily.
    if sign < 0.0 && nu.floor() != nu {
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

        let cy_k: Vec<Complex<f64>> = cy_kr
            .iter()
            .zip(cy_ki.iter())
            .map(|(&re, &im)| Complex::new(re, im))
            .collect();

        // In the case where kode == 2, i.e., we compute the exponentially scaled Bessel functions
        // ive and kve, we need to handle that the scaling is different for iv and kv.
        // That is,
        //
        //      ive(nu, z) = exp(-abs(z.re)) * iv(nu, z)    (see line 19 in zbesi.f),
        //
        //  and
        //
        //      kve(nu, z) = exp(z) * kv(nu, z)             (see line 21 in zbesk.f).
        //
        //  This means that for -nu < 0 with exponentially scaling we get
        //
        //      ive(-nu, z) = exp(-abs(z.re)) * iv(-nu, z)
        //                  = exp(-abs(z.re)) * (
        //                      iv(nu, z) + (2 / pi) * sin(pi * nu) kv(nu, z)
        //                  )
        //                  = ive(nu, z)
        //                      + (2 / pi) * sin(pi * nu) * exp(-abs(z.re)) * kv(nu, z)
        //                  = ive(nu, z) + (2 / pi) * sin(pi * nu)
        //                      * exp(-abs(z.re)) * exp(-z) * exp(z) * kv(nu, z)
        //                  = ive(nu, z) + (2 / pi) * sin(pi * nu)
        //                      * exp(-(z.re + abs(z.re)) - 1j * z.im) * kve(nu, z)

        let mut k_scaling = Complex::new(1.0, 0.0);

        if kode == 2 {
            k_scaling = (-z.re.abs() - z).exp();
        }

        // Handle lines 72-78 in zbesi.f
        for i in 0..(n as usize) {
            let sin_nupi = (std::f64::consts::PI * (nu + (i as f64))).sin();
            cy[i] = cy[i] + (2.0 / std::f64::consts::PI) * sin_nupi * k_scaling * cy_k[i];
        }
    }

    cy
}

pub fn iv_vec(nu: f64, z: Complex<f64>, n: i32) -> Vec<Complex<f64>> {
    let kode: i32 = 1;

    zbesi_vec(nu, z, kode, n)
}

pub fn ive_vec(nu: f64, z: Complex<f64>, n: i32) -> Vec<Complex<f64>> {
    let kode: i32 = 2;

    zbesi_vec(nu, z, kode, n)
}

pub fn iv_real_vec(nu: f64, z: f64, n: i32) -> Vec<f64> {
    let kode: i32 = 1;

    zbesi_vec(nu, Complex::new(z, 0.0), kode, n)
        .iter()
        .map(|&cy| cy.re)
        .collect()
}

pub fn ive_real_vec(nu: f64, z: f64, n: i32) -> Vec<f64> {
    let kode: i32 = 2;

    zbesi_vec(nu, Complex::new(z, 0.0), kode, n)
        .iter()
        .map(|&cy| cy.re)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_zbesi() {
        let kode: i32 = 1;
        let n: i32 = 3;

        let foo_vec = zbesi_vec(0.0, Complex::new(1.0, 1.0), kode, n);
        let foo = zbesi(0.0, Complex::new(1.0, 1.0), kode);

        assert_eq!(foo_vec.len(), n as usize);
        assert_abs_diff_eq!(foo_vec[0].re, foo.re);
        assert_abs_diff_eq!(foo_vec[0].im, foo.im);
    }

    #[test]
    fn test_iv_sym() {
        let n: i32 = 2;

        let foo_vec = iv_vec(-1.0, Complex::new(1.0, 1.0), n);
        let foo_2_vec = iv_vec(1.0, Complex::new(1.0, 1.0), n);

        assert_eq!(foo_vec.len(), n as usize);

        for i in 0..(n as usize) {
            assert_abs_diff_eq!(foo_vec[i].re, foo_2_vec[i].re);
            assert_abs_diff_eq!(foo_vec[i].im, foo_2_vec[i].im);
        }

        let foo = iv(-1.0, Complex::new(1.0, 1.0));

        assert_abs_diff_eq!(foo_vec[0].re, foo.re);
        assert_abs_diff_eq!(foo_vec[0].im, foo.im);
    }

    #[test]
    fn test_ive_sym() {
        let n: i32 = 3;

        let foo_vec = ive_vec(-1.0, Complex::new(1.0, 1.0), n);
        let foo_2_vec = ive_vec(1.0, Complex::new(1.0, 1.0), n);
        let foo = ive(-1.0, Complex::new(1.0, 1.0));

        assert_eq!(foo_vec.len(), n as usize);

        for i in 0..(n as usize) {
            assert_abs_diff_eq!(foo_vec[i].re, foo_2_vec[i].re);
            assert_abs_diff_eq!(foo_vec[i].im, foo_2_vec[i].im);
        }

        assert_abs_diff_eq!(foo_vec[0].re, foo.re);
        assert_abs_diff_eq!(foo_vec[0].im, foo.im);
    }
}
