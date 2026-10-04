use std::{env, process::Command};

fn main() {
    println!("cargo:rerun-if-env-changed=RUSTC");

    let rustc = env::var_os("RUSTC").expect("Cargo must provide RUSTC to build scripts");
    let output = Command::new(rustc)
        .arg("--version")
        .output()
        .expect("failed to query the Rust compiler version");
    assert!(output.status.success(), "rustc --version failed");

    let version = String::from_utf8(output.stdout).expect("rustc version must be UTF-8");
    println!("cargo:rustc-env=RUSTC_VERSION={}", version.trim());
}
