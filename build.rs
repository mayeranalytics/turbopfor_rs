fn main() {
    let deps_dir = "./deps";
    println!("cargo:rustc-link-search=native={}", deps_dir);
    println!("cargo:rustc-link-lib=static=ic");
}