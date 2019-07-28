#![allow(unused)]
pub struct S2 {}

pub struct X1(pub u8, pub u8, pub u8);
pub struct X2 {
    pub i: u8,
    pub j: u8,
    pub k: u8,
}

pub enum E {
    E0,
    E1,
    E2,
}

use E::*;

#[test]
fn test0() {
    let x1 = X1(1, 2, 3);
    let x2 = X2 { i: 1, j: 2, k: 3 };

    // overflow checks controlled globally by overflow-checked parameter
    let mut x: u8 = std::u8::MAX;
    // x += 1;
    let mut x: u8 = std::u8::MIN;
    // x -= 1;

    // using checked_{add,sub,mul,div}
    let mut x: u8 = std::u8::MAX;
    let x = x.checked_add(1);
    println!("{:?}", x); // None
    let mut x: u8 = std::u8::MAX - 1;
    let x = x.checked_add(1);
    println!("{:?}", x); // Some(255)
    let mut x: u8 = std::u8::MIN;
    let x = x.checked_sub(1);
    println!("{:?}", x); // None
    let mut x: u8 = std::u8::MIN + 1;
    let x = x.checked_sub(1);
    println!("{:?}", x); // Some(0)
    let mut x: u8 = 64;
    let x = x.checked_mul(4);
    println!("{:?}", x); // None
    let mut x: u8 = 64;
    let x = x.checked_mul(3);
    println!("{:?}", x); // Some(192)
    let mut x: u8 = 64;
    let x = x.checked_div(128);
    println!("{:?}", x); // Some(0)
    let mut x: u8 = 255;
    let x = x.checked_div(255);
    println!("{:?}", x); // Some(1)

    let mut x: u8 = std::u8::MAX;
    let x = x.wrapping_add(1);
    println!("{:?}", x); // 0
    let mut x: u8 = std::u8::MIN;
    let x = x.wrapping_sub(1);
    println!("{:?}", x); // 255

    let mut x: u8 = std::u8::MAX;
    let x = x.saturating_add(1);
    println!("{:?}", x); // 255
    let mut x: u8 = std::u8::MIN;
    let x = x.saturating_sub(1);
    println!("{:?}", x); // 0

    let _: f32 = std::f32::MIN;
    let _: f32 = std::f32::MAX;
    let _: f32 = std::f32::NAN;
    let _: f32 = std::f32::EPSILON;
    let _: f32 = std::f32::INFINITY;
    let _: f32 = std::f32::NEG_INFINITY;
    let _: f32 = std::f32::MIN_POSITIVE;

    let _: f64 = std::f64::MIN;
    let _: f64 = std::f64::MAX;
    let _: f64 = std::f64::NAN;
    let _: f64 = std::f64::EPSILON;
    let _: f64 = std::f64::INFINITY;
    let _: f64 = std::f64::NEG_INFINITY;
    let _: f64 = std::f64::MIN_POSITIVE;

    assert!(true);
    assert_eq!(0, -1 + 1);
}

/// main function
fn main() {
    println!("Hello, world!");
    // assert!(false);
    assert_eq!(0, -1 + 1);
}
