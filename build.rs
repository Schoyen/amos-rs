use cmake;
use std::env;

fn main() {
    let dst = cmake::build("amos");

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("amos").display()
    );

    if env::consts::OS.eq("macos") {
        // Hack to locate gfortran on Mac with gcc installed via Homebrew
        println!("cargo:rustc-link-search=/usr/local/lib/gcc/current/");
    }

    println!("cargo:rustc-link-lib=static=amos");
    // Link using gfortran instead of gcc, note this must come after the above command
    println!("cargo:rustc-link-lib=gfortran");
}
