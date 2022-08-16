cmake_minimum_required(VERSION 3.23.3)

project(amos-rs LANGUAGES C Fortran)

include(FortranCInterface)

FortranCInterface_HEADER(${CMAKE_CURRENT_SOURCE_DIR}/amos-mangle.h
    MACRO_NAMESPACE "F_"
    SYMBOL_NAMESPACE "f_"
)


set(SRC
    d1mach.f90
    fdump.f
    zabs.f
    zairy.f
    zbesi.f
    zbesy.f
    zbknu.f
    zdiv.f
    zlog.f
    zrati.f
    zshch.f
    zunhj.f
    zunik.f
    zuoik.f
    dgamln.f
    i1mach.f90
    zacai.f
    zasyi.f
    zbesj.f
    zbinu.f
    zbuni.f
    zexp.f
    zmlri.f
    zs1s2.f
    zsqrt.f
    zuni1.f
    zunk1.f
    zwrsk.f
    dsclmr.f
    r1mach.f90
    zacon.f
    zbesh.f
    zbesk.f
    zbiry.f
    zbunk.f
    zkscl.f
    zmlt.f
    zseri.f
    zuchk.f
    zuni2.f
    zunk2.f
)

add_library(amos STATIC ${SRC} amos-int.c)

install(TARGETS amos
    ARCHIVE DESTINATION ${CMAKE_INSTALL_PREFIX}/amos
)
