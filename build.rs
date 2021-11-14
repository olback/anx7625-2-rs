// cbindgen: Rust -> C
// bindgen: C -> Rust

fn main() {
    println!("cargo:rerun-if-changed=src/*");

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cbind_conf = cbindgen::Config::from_file("cbindgen.toml").unwrap();
    cbindgen::Builder::new()
        .with_config(cbind_conf)
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("gen/anx7625.h");

    bindgen::Builder::default()
        .header("driver/anx7625_pub.h")
        // .header("driver/edid.h")
        // .header("driver/anx7625.c")
        .use_core()
        .layout_tests(false)
        .size_t_is_usize(true)
        // .ctypes_prefix("ctypes")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .blocklist_type("AnxFnPtrs")
        .blocklist_type("__int8_t")
        .blocklist_type("__uint8_t")
        .blocklist_type("__int16_t")
        .blocklist_type("__uint16_t")
        .blocklist_type("__int32_t")
        .blocklist_type("__uint32_t")
        // .blocklist_item(".*")
        // .blocklist_type(".*")
        // .allowlist_type("struct edid")
        // .allowlist_type("edid")
        // .blocklist_function("*")
        .allowlist_type("edid")
        .allowlist_type("edid_modes")
        .allowlist_function("anx7625_.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("gen/anx7625.rs")
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .cpp(false)
        .files(&["driver/anx7625.c", "driver/edid.c"])
        .compile("libanx7625");
}
