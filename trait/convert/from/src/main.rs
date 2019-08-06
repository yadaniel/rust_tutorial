#![allow(unused)]

// only in lib
// #[stable(since="1.0.0")] 

/// traits: From, Into, TryFrom, TryInto
use std::convert::*;


/// From
/// One should always prefer implementing From over Into because implementing From automatically provides 
/// one with a implementation of Into thanks to the blanket implementation in the standard library.

#[derive(Debug)]
struct Num(u32);

impl From<u32> for Num {
    fn from(x:u32) -> Self {
        Num(x)
    }
}

#[test]
fn test1() {
    // manually implemented
    let n: Num = Num::from(1u32);
    match n {
        Num(x) => assert_eq!(x, 1u32)
    }
    // auto implemented for u32
    let v: Num = Into::<Num>::into(1u32);
}

/// Into
/// Only implement Into if a conversion to a type outside the current crate is required. 
/// From cannot do these type of conversions because of Rust's orphaning rules. See Into for more details.

struct Length(u32);

impl Into<Length> for u32 {
    fn into(self) -> Length {
        Length(self)
    }
}

#[test]
fn test2() {
    // manually implemented
    let n: Length = 1u32.into();
    // let n = 1u32.into<Length>();
    // auto implemented
    let v: Length = From::<u32>::from(1u32);
}

/// main

fn main() {
    let file = file!();
    println!("from trait example, file = {}, {}", file, file!());
}

