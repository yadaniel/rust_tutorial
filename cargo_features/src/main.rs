#![allow(unused)]

extern crate chrono;
// use chrono;

pub mod foo;

use libstrings::{Anystr, concat, concat_str, concat_string};

/// cargo run --features "with_colored"
/// use =, not == 
#[cfg(feature = "with_colored")]
fn log(txt: String) {
    extern crate colored;
    use colored::*;
    let timestamp = chrono::Utc::now().to_string();
    println!("{}:{}", timestamp.green(), txt.blue().bold());
}

/// cargo run
#[cfg(not(feature = "with_colored"))]
fn log(txt: String) {
    let timestamp = chrono::Utc::now().to_string();
    println!("{}:{}", timestamp, txt);
}

fn main() {
    let f = foo::Foo::new(1,2,3);
    println!("{:?}", f);
    // use libstrings crate
    println!("{}", concat_string("test".to_string(), "_it!".to_string()));

    println!("{}", libstrings::strip_left("   some text".to_string()));

    // test feature
    log("log text".to_string());
}

