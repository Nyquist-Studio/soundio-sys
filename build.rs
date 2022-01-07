use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    let soundio_build_output = build_and_link_soundio();
    generate_rust_binding(&soundio_build_output);
}

fn build_and_link_soundio() -> PathBuf {
    let soundio_build_output = cmake::Config::new("./libsoundio/")
        .define("BUILD_STATIC_LIBS", "ON")
        .define("BUILD_DYNAMIC_LIBS", "OFF")
        .define("BUILD_EXAMPLE_PROGRAMS", "OFF")
        .define("BUILD_TESTS", "OFF")
        .define("ENABLE_JACK", "ON")
        .define("ENABLE_PULSEAUDIO", "ON")
        .define("ENABLE_ALSA", "ON")
        .define("ENABLE_COREAUDIO", "ON")
        .define("ENABLE_WASAPI", "ON")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        soundio_build_output.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=soundio");

    #[cfg(target_os = "macos")]
    link_macos_system_frameworks();

    soundio_build_output
}

#[cfg(target_os = "macos")]
fn link_macos_system_frameworks() {
    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    println!("cargo:rustc-link-lib=framework=CoreAudio");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
}

fn generate_rust_binding(build_output: &Path) {
    let umbrella_header_path = build_output
        .join("include")
        .join("soundio")
        .join("soundio.h");

    println!("cargo:rerun-if-changed={}", umbrella_header_path.display());

    let bindings = bindgen::Builder::default()
        .header(umbrella_header_path.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("bindings.rs");

    if let Err(error) = bindings.write_to_file(out_path) {
        println!("cargo:warning={}", error);
    }
}
