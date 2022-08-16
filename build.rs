use cmake;

fn main() {
    let dst = cmake::build("amos");

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("amos").display()
    );

    println!("cargo:rustc-link-lib=static=amos");
    // Link using gfortran instead of gcc, note this must come after the above command
    println!("cargo:rustc-link-lib=gfortran");
}
