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
licensed](https://github.com/certik/fortran-utils/blob/b43bd24cd421509a5bc6d3b9c3eeae8ce856ed88/LICENSE).
However, as these authors downloaded much of the code from
[SciPy](https://github.com/scipy/scipy/tree/7b5ad7a6a921a845bdb170d48c54ab12a45af616/scipy/special/amos)
there is also a corresponding [BSD-3-Clause
license](https://github.com/scipy/scipy/blob/27aaa296daf8f5a81beeb6504ae405719abee626/LICENSE.txt).
Finally, in the [README.md](amos/README.md) and the [dsclmr.f](amos/dsclmr.f)
there is a statement on the unlimited release of the original AMOS code.
The [dsclmr.f](amos/dsclmr.f)-file can be found on Netlib via
[https://netlib.org/amos/dsclmr.f](https://netlib.org/amos/dsclmr.f) (note that
clicking this link downloads the file).

The Rust-wrapper code is licensed under [MIT](LICENSE).
