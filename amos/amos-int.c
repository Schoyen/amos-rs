// #include <stdio.h>

#include "amos-mangle.h"


extern void F_GLOBAL(zbesh, ZBESH)(
        const double *zr,
        const double *zi,
        const double *fnu,
        const int *kode,
        const int *m,
        const int *n,
        double *cyr,
        double *cyi,
        int *nz,
        int *ierr
);

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

extern void F_GLOBAL(zbesj, ZBESJ)(
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

extern void F_GLOBAL(zbesk, ZBESK)(
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

extern void F_GLOBAL(zbesy, ZBESY)(
        const double *zr,
        const double *zi,
        const double *fnu,
        const int *kode,
        const int *n,
        double *cyr,
        double *cyi,
        int *nz,
        double *cwrkr,
        double *cwrki,
        int *ierr
);

extern void F_GLOBAL(zairy, ZAIRY)(
        const double *zr,
        const double *zi,
        const int *id,
        const int *kode,
        double *air,
        double *aii,
        int *nz,
        int *ierr
);

extern void F_GLOBAL(zbiry, ZBIRY)(
        const double *zr,
        const double *zi,
        const int *id,
        const int *kode,
        double *bir,
        double *bii,
        int *ierr
);


extern double F_GLOBAL(dgamln, DGAMLN)(
        const double *z,
        int *ierr
);
