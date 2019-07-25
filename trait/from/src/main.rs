#![allow(unused)]

mod a {}    // local module
mod b;      // external file module
mod c;      // external file in directory module

#[derive(PartialEq,Eq)]
#[derive(PartialOrd,Ord)]
#[derive(Clone, Copy)]      // copy instead move ... copy is a marker trait and is derived from clone
#[derive(Debug)]
struct Num(u64);

// Display trait has no auto implementation
use std::fmt;
impl fmt::Display for Num {
    //fn fmt(self: &Self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

// note: here convert u8 -> Num
// implementing From<u8> for Num trait 
// auto generates Into<Num> for u8
impl From<u8> for Num {
    fn from(x:u8) -> Num {
        Num(x as u64)
    }
}
//
// uncomment Into<Num> then compiler will generated error message - confilicting implementation
//impl Into<Num> for u8 {
//    fn into(self) -> u8 { self.0 as u8 }
//}


// note: convert Num -> u8
impl From<Num> for u8 {
    fn from(x:Num) -> u8 {
        match x {
            Num(v) => v as u8
        }
    }
}

impl Num {
    // static functions
    fn new(val: u64) -> Num { Num(val) }
    fn zero() -> Num { Num(0) }
    // instance functions
    fn print(&self) { println!("<{}>", self.0); }
}

fn main() {
    let n1 = Num::new(0);
    let n2 = Num::zero();
    let b = n1 == n2;
    println!("{} == {} is {}", n1, n2, b);

    n1.print();         // instance call syntax
    Num::print(&n1);    // universal call syntax

    // here u8 -> Num
    let n3 = Num::from(1u8);            // implemented
    let n3 : Num = 1u8.into();          // auto generated
    let n3 = Into::<Num>::into(1u8);    // auto generated
    // not implemented
    //let n4 = Num::from(1u16); // trait not implemented for u16
    
    // here Num -> u8
    let u1 = u8::from(Num::new(3));
    let u2 : u8 = n3.into();
    let u3 = Into::<u8>::into(Num::new(7));
    println!("{},{},{}", u1, u2, u3);
    
    let n5 = Num::new(10);
    let n6 = n5;    // n5 moved, when copy trait not implemented (comment out Copy, Clone)
    println!("{}, {}", n5, n6);

}

