use cmake;


fn main() {
    let dst = cmake::build("amos-c");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=foo");
}
