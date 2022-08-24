# Amos wrapper in Rust

![](https://github.com/Schoyen/amos-rs/actions/workflows/rust.yml/badge.svg)

This is a wrapper for Amos' [algorithm
644](https://dl.acm.org/doi/10.1145/7921.214331) written in Rust.
The original Fortran 77 code can be found at
[netlib](https://netlib.org/amos/).



## License

The legacy AMOS code is downloaded from
[fortran-utils](https://github.com/certik/fortran-utils/tree/70b239f0e474ffd0ea407a9b20f49d93f34e4c27/src/legacy/amos),
which is [MIT
licensed](https://github.com/certik/fortran-utils/blob/b43bd24cd421509a5bc6d3b9c3eeae8ce856ed88/LICENSE),
and [SciPy](https://doi.org/10.1145/305658.305711) which is licensed under [BSD-3-Clause
license](https://github.com/scipy/scipy/blob/27aaa296daf8f5a81beeb6504ae405719abee626/LICENSE.txt).
In particular routines such as, e.g., `ZABS` in [zabs.f](amos/zabs.f) have been
renamed to `AZABS` to avoid conflict with built-in functions in modern Fortran.
These are the main differences in the derived Amos source code and the one
found at [netlib.org/amos](netlib.org/amos).

The files [d1mach.f90](amos/d1mach.f90), [i1mach.f90](amos/i1mach.f90), and
[r1mach.f90](amos/r1mach.f90) are found at
[https://wg25.taa.univie.ac.at/ifip/kyoto/workshop-info/proceedings/einarsson/d1mach.html](https://wg25.taa.univie.ac.at/ifip/kyoto/workshop-info/proceedings/einarsson/d1mach.html).
They can be downloaded at [r1mach.f90](https://wg25.taa.univie.ac.at/ifip/kyoto/workshop-info/proceedings/einarsson/f90/r1mach.f90), [d1mach.f90](https://wg25.taa.univie.ac.at/ifip/kyoto/workshop-info/proceedings/einarsson/f90/d1mach.f90), and [i1mach.f90](https://wg25.taa.univie.ac.at/ifip/kyoto/workshop-info/proceedings/einarsson/f90/i1mach.f90).
The paper describing these implementations are found at [https://doi.org/10.1145/305658.305711](https://doi.org/10.1145/305658.305711).

Finally, in the [README.md](amos/README.md) and the [dsclmr.f](amos/dsclmr.f)
there is a statement on the unlimited release of the original AMOS code.
The [dsclmr.f](amos/dsclmr.f)-file can be found on Netlib via
[https://netlib.org/amos/dsclmr.f](https://netlib.org/amos/dsclmr.f) (note that
clicking this link downloads the file).

The Rust-wrapper code is licensed under [MIT](LICENSE).
