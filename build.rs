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
        .header("driver/anx7625.c")
        .use_core()
        // .blocklist_item("*")
        .blocklist_type("*")
        // .blocklist_function("*")
        .allowlist_function("anx_.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("gen/anx7625.rs")
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .cpp(false)
        .file("driver/anx7625.c")
        .compile("libanx7625");
}
