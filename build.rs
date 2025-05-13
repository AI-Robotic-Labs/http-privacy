use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++17")
        .compile("privacy_http_sdk");

    // Copiar o header gerado para src/
    let header_src = out_dir.join("lib.rs.h");
    let header_dst = PathBuf::from("src/lib.rs.h");

    if header_src.exists() {
        fs::copy(&header_src, &header_dst)
            .expect("Fail to the copy lib.rs.h for src/");
        println!("cargo:warning=lib.rs.h copied with sucess!");
    } else {
        panic!("Achive lib.rs.h not was generated!");
    }

    println!("cargo:rerun-if-changed=src/lib.rs");
}
