#![allow(unused)]

// all the lines printed to stdout by a build script are written to a file like 
// target/debug/build/<pkg>/output
// target/debug/build/build_script-342627de92f54e22/output
// find target/debug -iname output -exec cat {} \;
//
// if you want to ensure that output is always displayed on your terminal. 
// Any line that starts with cargo: is interpreted directly by Cargo. 
// This line must be of the form cargo:key=value, like the examples below
//  cargo:rustc-link-lib=static=foo
//  cargo:rustc-link-search=native=/path/to/foo
//  cargo:rustc-cfg=foo
//  cargo:rustc-env=FOO=bar
//  cargo:rustc-cdylib-link-arg=-Wl,-soname,libfoo.so.1.2.3
//  # arbitrary user-defined metadata
//  cargo:root=/path/to/foo
//  cargo:libdir=/path/to/foo/lib
//  cargo:include=/path/to/foo/include

// usage
// cargo clean; WHAT=foo cargo run
// cargo clean; WHAT=bar cargo run
// cargo clean; WHAT=foobar cargo run

fn main() {
    let dir = env!("PWD");
    let what = env!("WHAT");
    println!("building in {} with {}", dir, what);
    match what {
        "foo" => println!("cargo:rustc-cfg=foo"),
        "bar" => println!("cargo:rustc-cfg=bar"),
        "foobar" => println!("cargo:rustc-cfg=foobar"),
        _ => panic!("unknown")
    }
}

