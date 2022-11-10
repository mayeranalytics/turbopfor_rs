use std::process::Command;

fn main() {
    Command::new("make").status().unwrap();
    println!("cargo:rustc-link-search=native={}", "./vendor/turbopfor");
    println!("cargo:rustc-link-lib=static=ic");
}
