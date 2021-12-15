extern crate cmake;

use cmake::Config;
use std::env;
use std::process::Command;

fn main() {
    let mut cmake = Config::new("solidity");
    cmake
        .define("TESTS", "OFF")
        .define("TOOLS", "OFF")
        .define("USE_Z3", "OFF")
        .define("USE_CVC4", "OFF")
        .define("Boost_USE_STATIC_LIBS", "ON")
        .cxxflag("-Wno-range-loop-analysis");

    if Command::new("sccache").arg("--version").output().is_ok() {
        cmake
            .define("CMAKE_CXX_COMPILER_LAUNCHER", "sccache")
            .define("CMAKE_C_COMPILER_LAUNCHER", "sccache");
    }
    let dst = cmake.build();

    for lib in vec![
        "solc", "solidity", "yul", "langutil", "evmasm", "solutil", "smtutil",
    ] {
        println!(
            "cargo:rustc-link-search=native={}/build/lib{}",
            dst.display(),
            lib
        );
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // jsoncpp dependency
    println!(
        "cargo:rustc-link-search=native={}/build/deps/lib",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=jsoncpp");

    println!("cargo:rustc-link-search=native=/usr/lib/");
    println!("cargo:rustc-link-search=native=/usr/lib64/");
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-search=native=/usr/local/lib/");

    println!("cargo:rustc-link-lib=static=boost_system");
    println!("cargo:rustc-link-lib=static=boost_filesystem");
    println!("cargo:rustc-link-lib=static=boost_regex");

    // We need to link against C++ std lib
    if let Some(cpp_stdlib) = get_cpp_stdlib() {
        println!("cargo:rustc-link-lib={}", cpp_stdlib);
    }
}

// See https://github.com/alexcrichton/gcc-rs/blob/88ac58e25/src/lib.rs#L1197
fn get_cpp_stdlib() -> Option<String> {
    env::var("TARGET").ok().and_then(|target| {
        if target.contains("msvc") {
            None
        } else if target.contains("darwin") {
            Some("c++".to_string())
        } else if target.contains("freebsd") {
            Some("c++".to_string())
        } else if target.contains("musl") {
            Some("static=stdc++".to_string())
        } else {
            Some("stdc++".to_string())
        }
    })
}
