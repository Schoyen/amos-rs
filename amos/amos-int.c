// #include <stdio.h>

#include "amos-mangle.h"


extern void F_GLOBAL(zbesi, ZBESI)(
        const double *zr,
        const double *zi,
        const double *fnu,
        const int *kode,
        const int *n,
        double *cyr,
        double *cyi,
        int *nz,
        int *ierr
);


// void zbesi_w(
//         const double *zr,
//         const double *zi,
//         const double *fnu,
//         const int *kode,
//         const int *n,
//         double *cyr,
//         double *cyi,
//         int *nz,
//         int *ierr
// )
// {
//     printf("%f, %f, %f, %d, %d\n", *zr, *zi, *fnu, *kode, *n);
//     F_GLOBAL(zbesi, ZBESI)(zr, zi, fnu, kode, n, cyr, cyi, nz, ierr);
// }
