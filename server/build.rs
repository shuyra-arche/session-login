//! Link the `login` image with the script published by the execution package.

fn main() {
    let ld = std::env::var("DEP_EXECUTION_LINKER_SCRIPT")
        .expect("execution did not publish DEP_EXECUTION_LINKER_SCRIPT");
    println!("cargo::rustc-link-arg-bin=login=-T{ld}");
    println!("cargo::rerun-if-changed={ld}");

    println!("cargo::rerun-if-changed=src/main.rs");
    println!("cargo::rerun-if-changed=src/auth.rs");
    println!("cargo::rerun-if-changed=src/startup/mod.rs");
    println!("cargo::rerun-if-changed=src/startup/context.rs");
    println!("cargo::rerun-if-changed=src/startup/issuer.rs");
}
