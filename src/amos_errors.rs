use std::os::raw::c_int;

use log::warn;

pub fn handle_ierr(func: &str, nz: c_int, ierr: c_int) {
    if nz == 0 && ierr == 0 {
        return;
    }

    if nz > 0 {
        warn!(
            "Underflow occured in {} with {} digits set to zero",
            func, nz
        );
    }

    if ierr == 0 {
        // Normal return
        return;
    }

    assert!(nz >= 0);
    assert!(ierr >= 0 && ierr <= 5);

    if ierr == 1 {
        // This error should never occur as the handling of input is done in the wrapper functions.
        // Unrecoverable error, wrapper library error.
        panic!("Input error in {}", func);
        // Unreachable
        return;
    }

    if ierr == 2 {
        // This error is handled in the respective functions as the result will typically be +/-
        // INF, making the error recoverable.
        warn!("Overflow occured in {}", func);
        return;
    }

    if ierr == 3 {
        // Recoverable situation, only issue warning.
        warn!("Loss of significance in {}, z or nu + n - 1 is large", func);
        return;
    }

    if ierr == 4 {
        // Recoverable situation, only issue warning.
        // The calling function should return nan.
        warn!(
            "Complete loss of significan in {}, z or nu + n - 1 is too large",
            func
        );
        return;
    }

    if ierr == 5 {
        // Recoverable situation, only issue warning.
        // The calling function should return nan.
        warn!("Termination condition not met in {}", func);
        return;
    }
}
