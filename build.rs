// Windows gives the main thread only 1 MB of stack, where Linux defaults to
// 8 MB. Vulkano's deeply-generic builders and the `vulkano_shaders::shader!`
// output blow past 1 MB in debug builds, which shows up at startup as
// STATUS_STACK_OVERFLOW (0xc00000fd). Reserve 16 MB for the main thread at
// link time so the same code runs on both platforms.
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if std::env::var("CARGO_CFG_WINDOWS").is_err() {
        return;
    }

    match std::env::var("CARGO_CFG_TARGET_ENV").as_deref() {
        Ok("msvc") => println!("cargo:rustc-link-arg-bins=/STACK:16777216"),
        _ => println!("cargo:rustc-link-arg-bins=-Wl,--stack,16777216"),
    }
}
