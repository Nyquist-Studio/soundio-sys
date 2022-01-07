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
        .define("BUILD_STATIC_LIBS", "true")
        .define("BUILD_DYNAMIC_LIBS", "false")
        .define("BUILD_EXAMPLE_PROGRAMS", "false")
        .define("BUILD_TESTS", "false")
        .define("ENABLE_JACK", "false")
        .define("ENABLE_PULSEAUDIO", "false")
        .define("ENABLE_ALSA", "false")
        .define("ENABLE_COREAUDIO", "true")
        .define("ENABLE_WASAPI", "false")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        format!("{}/lib", soundio_build_output.display())
    );
    println!("cargo:rustc-link-lib=static=soundio");

    soundio_build_output
}

fn generate_rust_binding(build_output: &Path) {
    let umbrella_header_path = format!("{}/include/soundio/soundio.h", build_output.display());

    println!("cargo:rerun-if-changed={}", umbrella_header_path);

    let bindings = bindgen::Builder::default()
        .header(umbrella_header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("bindings.rs");

    bindings
        .write_to_file(out_path)
        .expect("Failed to write bindings");
}
