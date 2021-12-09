
// Copyright 2020 Self Group Ltd. All Rights Reserved.

extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

//use autotools::Config;

fn main() {
    // Tell cargo to tell rustc to link the system olm
    // shared library.

    let target = env::var("TARGET").unwrap();

    if target == "x86_64-apple-darwin" {

    } else if target == "x86_64-apple-ios" {

    } else if target == "x86_64-linux-android" {

    } else if target == "x86_64-unknown-linux-gnu" {

    }
    

    println!("{}", env::var("OUT_DIR").unwrap());

    Command::new("./configure")
        // .arg(format!("--prefix=:{}/", env::var("OUT_DIR").unwrap()))
        .current_dir("vendor")
        .output()
        .expect("failed to configure");


   Command::new("make")
        .arg("dep")
        .current_dir("vendor")
        .output()
        .expect("failed to make deps");

    Command::new("make")
        .arg("clean")
        .current_dir("vendor")
        .output()
        .expect("failed to make clean");
    

    Command::new("make")
        .current_dir("vendor")
        .output()
        .expect("failed to make");


    println!("cargo:rerun-if-changed=zrtp.h");

    // generate the bindings for pjproject headers
    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivendor/pjlib/include/")
        .clang_arg("-Ivendor/pjlib-util/include/")
        .clang_arg("-Ivendor/pjnath/include/")
        .clang_arg("-Ivendor/pjmedia/include/")
        .header("pjproject.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate pjproject bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // output the bindings
    bindings
        .write_to_file(out_path.join("pjproject.rs"))
        .expect("Couldn't write pjproject bindings!");

}