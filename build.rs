use std::{env, fs, path::PathBuf};

fn main() {
    let engine = PathBuf::from("upstream/calculator/src/CalcManager");
    assert!(
        engine.join("CalculatorManager.cpp").is_file(),
        "Initialize original sources with: git submodule update --init --recursive"
    );
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .warnings(false)
        .include(&engine)
        .include(engine.join("Header Files"))
        .include(engine.join("Ratpack"));
    // Supply platform headers outside the original source tree. MSVC uses its SDK.
    if !env::var("CARGO_CFG_TARGET_ENV").is_ok_and(|v| v == "msvc") {
        build.include("native/compat");
        build.flag("-include").flag("native/compat/portable.h");
    } else {
        build.flag("/utf-8");
        build.flag("/FI").flag("native/compat/portable.h");
    }
    for dir in [
        engine.clone(),
        engine.join("CEngine"),
        engine.join("Ratpack"),
    ] {
        let mut paths: Vec<_> = fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        paths.sort();
        for path in paths {
            if path.extension().is_some_and(|e| e == "cpp") && !path.ends_with("pch.cpp") {
                build.file(path);
            }
        }
    }
    build
        .include(env::var("OUT_DIR").unwrap())
        .file("native/bridge.cpp");
    build.compile("calculator_original");
    println!("cargo:rerun-if-changed=upstream/calculator/src/CalcManager");
    println!("cargo:rerun-if-changed=native");
}
