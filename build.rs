use std::env;

fn main() {
    println!(
        "cargo:rustc-env=DWPROD_EXE={}",
        if env::var("PROFILE").unwrap() == "release" {
            concat!(env!("CARGO_MANIFEST_DIR"), "/target/release/dwprod")
        } else {
            concat!(env!("CARGO_MANIFEST_DIR"), "/target/debug/dwprod")
        }
    );
}
