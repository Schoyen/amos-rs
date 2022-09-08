pub mod amos_errors;
mod bindings;
pub mod zbesh;
pub mod zbesi;

#[cfg(test)]
mod tests {
    use super::*;
    use bindings::*;
    use std::os::raw::c_int;

    #[test]
    fn test_zbesi() {
        let zr = 0.1;
        let zi = 0.0;
        let fnu = 0.0;
        let kode: c_int = 1;
        let n: c_int = 1;

        let mut cyr = 0.0;
        let mut cyi = 0.0;
        let mut nz: c_int = 0;
        let mut ierr: c_int = 0;

        unsafe {
            zbesi_(
                &zr,
                &zi,
                &fnu,
                &kode as *const c_int,
                &n as *const c_int,
                &mut cyr,
                &mut cyi,
                &mut nz as *mut c_int,
                &mut ierr as *mut c_int,
            );
        }

        assert_eq!(0, nz);
        assert_eq!(0, ierr);
        // assert_eq!(1.0, cyr);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
