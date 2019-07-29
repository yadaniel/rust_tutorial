#![allow(unused)]

pub struct S0;
pub struct S1();
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

fn take_slice(slc: &[u8]) -> &[u8] {
    // same slice
    // slc
    // &slc
    // &slc[..]
    // &slc[0..slc.len()]

    // part of the slice
    // &slc[..slc.len() / 2] // the lower half
    &slc[slc.len() / 2..] // the upper half
}

fn take_mut_slice(slc: &mut [u8]) -> &mut [u8] {
    let n = slc.len();
    for i in 0..n / 2 {
        let tmp = slc[i];
        slc[i] = slc[n - 1 - i];
        slc[n - 1 - i] = tmp;
    }
    slc
}

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

static mut CNT: u8 = 0;
fn get_value() -> u8 {
    unsafe {
        // CNT += 1;   // will overflow
        CNT = CNT.wrapping_add(1);
        CNT
    }
}

/// main function
fn main() {
    println!("Hello, world!");
    // assert!(false);
    assert_eq!(0, -1 + 1);
    //
    let buffer: [u8; 10] = { [1; 10] };
    println!("{:?}", buffer);
    let buffer: [u8; 10] = { [get_value(); 10] }; // is called only once
    println!("{:?}", buffer);
    let buffer: [u8; 10] = {
        let mut tmp: [u8; 10] = [0; 10];
        for idx in 0..10 {
            tmp[idx] = get_value();
        }
        tmp
    };
    println!("{:?}", buffer);

    let buffer: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v = take_slice(&buffer);
    println!("{:?}", v);
    let v = take_slice(&buffer[..]);
    println!("{:?}", v);
    let v = take_slice(&buffer[0..]); // inclusive
    println!("{:?}", v);
    let v = take_slice(&buffer[..4]); // exclusive
    println!("{:?}", v);
    let v = take_slice(&buffer[..=4]); // inclusive
    println!("{:?}", v);
    let v = take_slice(&buffer[4..8]);
    println!("{:?}", v);

    let mut buffer: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v = take_mut_slice(&mut buffer);
    println!("{:?}", v);
    let v = take_slice(&buffer[4..8]);
    println!("{:?}", v);

    let x: [Option<u8>; 10] = [None; 10];
    let x: [Option<u8>; 10] = [Some(0); 10];

    #[derive(Copy, Clone)]
    enum ERR {
        NoError,
        ErrorFile(u8),
        ErrorValue(u8),
    };
    let x: [Result<u8, u8>; 10] = [Ok(0); 10];
    let x: [Result<u8, ERR>; 10] = [Err(ERR::NoError); 10];
}
