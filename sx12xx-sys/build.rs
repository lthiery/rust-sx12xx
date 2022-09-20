//#[cfg(workaround_build)]
fn main() {
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;
    use cmake;
    use cmake::Config;

    let dst = Config::new("sx12xx")
                 .define("BUILD_TESTING", "OFF")
                 .define("CMAKE_C_COMPILER_WORKS", "1")
                 .define("CMAKE_CXX_COMPILER_WORKS", "1")
                 .pic(false)
                 .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=sx12xx");

   // make the bindings
   let bindings = bindgen::Builder::default()
       .raw_line("use cty;")
       .use_core()
       .ctypes_prefix("cty")
       .detect_include_paths(true)
       .header("sx12xx/sx12xx.h")
       .header("sx12xx/board.h")
       .header("sx12xx/radio.h")
       .header("sx12xx/sx1276/sx1276.h")
       .header("sx12xx/sx1272/sx1272.h")
       .header("sx12xx/sx126x/sx126x.h")
       .clang_arg(format!("-I{}/include",dst.display()))
       .trust_clang_mangling(false)
       .rustfmt_bindings(true)
       .allowlist_type("Radio_t")
       .allowlist_type("Sx12xxEvent_t")
       .allowlist_type("Sx12xxState_t")
       .allowlist_type("AntPinsMode_t")
       .allowlist_type("RadioModems_t")
       .allowlist_type("Sx12xx_t")
       .allowlist_type("Sx12xxRxMetadata_t")
       .rustified_enum("Sx12xxEvent_t")
       .rustified_enum("Sx12xxState_t")
       .rustified_enum("AntPinsMode_t")
       .allowlist_function("SX1276RadioNew")
       .allowlist_function("SX126xRadioNew")
       .allowlist_function("sx12xx_init")
       .allowlist_function("sx12xx_new_handle")
       .allowlist_function("sx12xx_handle_event")
       .allowlist_function("sx12xx_send")
       .allowlist_function("sx12xx_set_rx_buffer")
       .allowlist_function("sx12xx_get_rx_metadata")
       .allowlist_function("sx12xx_get_raw_buffer")
       .derive_copy(false)
       .derive_debug(false)
       .layout_tests(false)
       .generate()
       .expect("Failed to generate sx1276 bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

// #[cfg(not(workaround_build))]
// fn main() {
//   cargo_5730::run_build_script();
// }

