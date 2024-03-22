use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=lib/*");
    Command::new("flutter build");
}
