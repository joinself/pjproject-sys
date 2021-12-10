
// Copyright 2020 Self Group Ltd. All Rights Reserved.

extern crate bindgen;
extern crate make_cmd;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {

    let target = env::var("TARGET").unwrap();

    
    

    if target == "x86_64-apple-darwin" {
        
    } else if target == "x86_64-apple-ios" {

    } else if target == "x86_64-linux-android" {

    } else if target == "x86_64-unknown-linux-gnu" {
    }

    Command::new("./configure")
        .arg("--disable-ssl")
        .arg("--disable-sound")
        .arg("--disable-resample")
        .arg("--disable-small-filter")
        .arg("--disable-large-filter")
        .arg("--disable-speex-aec")
        .arg("--disable-g711-plc")
        .arg("--disable-g711-codec")
        .arg("--disable-gsm-codec")
        .arg("--disable-speex-codec")
        .arg("--disable-ilbc-codec")
        .arg("--disable-floating-point")
        .current_dir("vendor")
        .output()
        .expect("failed to configure");


    make_cmd::make()
        .arg("dep")
        .current_dir("vendor")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    
    make_cmd::make()
        .arg("clean")
        .current_dir("vendor")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    make_cmd::make()
        .current_dir("vendor")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    println!("cargo:rerun-if-changed=zrtp.h");

    // generate the bindings for pjproject headers
    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivendor/pjlib/include/")
        .clang_arg("-Ivendor/pjlib-util/include/")
        .clang_arg("-Ivendor/pjnath/include/")
        .clang_arg("-Ivendor/pjmedia/include/")
        .clang_arg("-Ivendor/pjsip/include/")
        .allowlist_type(r"pj.*")
        .allowlist_type(r"PJ.*")
        .allowlist_function(r"pj.*")
        .allowlist_function(r"PJ.*")
        .allowlist_var(r"pj.*")
        .allowlist_var(r"PJ.*")
        .header("pjproject.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate pjproject bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // output the bindings
    bindings
        .write_to_file(out_path.join("pjproject.rs"))
        .expect("Couldn't write pjproject bindings!");


    // link generated libraries
    
    println!("cargo:rustc-link-search=vendor/pjlib/lib/");
    println!("cargo:rustc-link-lib=pj-{}", target);

    println!("cargo:rustc-link-search=vendor/pjlib-util/lib/");
    println!("cargo:rustc-link-lib=pjlib-util-{}", target);

    println!("cargo:rustc-link-search=vendor/pjmedia/lib/");
    println!("cargo:rustc-link-lib=pjmedia-{}", target);
    println!("cargo:rustc-link-lib=pjmedia-audiodev-{}", target);
    println!("cargo:rustc-link-lib=pjmedia-codec-{}", target);
    println!("cargo:rustc-link-lib=pjmedia-videodev-{}", target);
    println!("cargo:rustc-link-lib=pjsdp-{}", target);

    println!("cargo:rustc-link-search=vendor/pjnath/lib/");
    println!("cargo:rustc-link-lib=pjnath-{}", target);

    println!("cargo:rustc-link-search=vendor/pjsip/lib/");
    println!("cargo:rustc-link-lib=pjsip-{}", target);
    println!("cargo:rustc-link-lib=pjsip-ua-{}", target);
    println!("cargo:rustc-link-lib=pjsip-simple-{}", target);
    println!("cargo:rustc-link-lib=pjsua-{}", target);
    println!("cargo:rustc-link-lib=pjsua2-{}", target);

    println!("cargo:rustc-link-lib=uuid");
}