#![allow(unused)]

pub mod foo;

use libstrings::{Anystr, concat, concat_str, concat_string};

fn main() {
    let f = foo::Foo::new(1,2,3);
    println!("{:?}", f);
    // use libstrings crate
    println!("{}", concat_string("test".to_string(), "_it!".to_string()));

    println!("{}", libstrings::strip_left("   some text".to_string()));

    // let mut found = false;
    // let mut s = String::new();
    // for c in "   some text".chars() {
    //     match c {
    //         ' '|'\t' if !found => continue,
    //         _ => { found = true; s.push(c); },
    //     }
    // }
    // println!("{}", s);

}

