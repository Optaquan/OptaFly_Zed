fn main() {
    // Rerun if lib.rs changes
    println!("cargo:rerun-if-changed=src/lib.rs");

    // Set rpath for Linux/macOS to find dependencies
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
}
