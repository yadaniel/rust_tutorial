#![allow(unused)]

/// the configuration is selected in build.rs with
/// println!("cargo:rustc-cfg=foobar");

#[cfg(foo)]
fn foo_bar() {
    println!("in foo");
}

#[cfg(bar)]
fn foo_bar() {
    println!("in bar");
}

#[cfg(foobar)]
fn foo_bar() {
    println!("in foobar");
}

// cargo clean; RUSTFLAGS="--cfg=foo" WHAT=foobar cargo run
// the name `foo_bar` is defined multiple times

// #[cfg(any(foo,bar))]
#[cfg(any(not(foobar)))]
fn xyz() {
    println!("in xyz");
}

#[cfg(foobar)]
fn xyz() {
    println!("in xyz from foobar");
}

// cargo clean; RUSTFLAGS="--cfg=system_lib" WHAT=foobar cargo run
// the name `xyz` is defined multiple times
#[cfg(system_lib)]
fn xyz() {
    println!("in xyz from system_lib");
}

fn main() {
    foo_bar();
    xyz();

    // match on str
    let line: &str = "one";
    let line: &str = "two";
    let line: &str = "three";
    match line {
        "one" => println!("1"),
        "two" => println!("2"),
        "three" => println!("3"),
        _ => println!("_"),
    }

}

