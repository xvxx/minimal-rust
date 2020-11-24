//! Counts the total number of dependencies for a crate.
//! usage: $ cargo run --bin count-deps CRATE

use std::{
    fs,
    io::Write,
    process::{exit, Command},
};

const TMPDIR: &str = "/tmp/minimal-rust-dep-counter";

const MAIN_RS: &str = "fn main() {}";

const CARGO_MANIFEST: &str = r#"
[package]
name = "minimal-rust-dep-counter"
version = "0.0.1"
authors = ["you <dep-counter@example.com>"]
edition = "2018"

[dependencies]
$crate = { version = "*", default-features = false }
"#;

fn main() {
    if let Some(name) = std::env::args().nth(1) {
        let _ = fs::remove_dir_all(TMPDIR);
        let _ = fs::create_dir_all(TMPDIR);
        let _ = fs::create_dir_all(&format!("{}/src", TMPDIR));
        let mut cargo = fs::File::create(&format!("{}/src/main.rs", TMPDIR)).unwrap();
        cargo.write_all(MAIN_RS.as_bytes()).unwrap();
        let mut cargo = fs::File::create(&format!("{}/Cargo.toml", TMPDIR)).unwrap();
        cargo
            .write_all(CARGO_MANIFEST.replace("$crate", &name).as_bytes())
            .unwrap();

        let output = Command::new("sh")
            .args(&[
                "-c",
                &format!(
                    "cd {} && cargo tree -p {} -e no-dev --no-dedupe --prefix none | sort | uniq | wc -l",
                    TMPDIR,
                    name,
                ),
            ])
            .output()
            .unwrap();
        let count = String::from_utf8_lossy(&output.stdout);
        let count = count.trim().parse::<usize>().unwrap();
        if count > 0 {
            println!("{}'s total dependency count: {}", name, count - 1);
        } else {
            eprintln!("Crate `{}` doesn't exist", name);
            exit(1);
        }
        let _ = fs::remove_dir_all(TMPDIR);
    } else {
        eprintln!("please provide a crate name");
        exit(1);
    }
}
