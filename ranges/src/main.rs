#![allow(unused)]

// inclusive range
// in match, for loop and slices => low..=high
// exclusive range
// in match => low...high
// in for loop and slices => low..high

fn types_and_literals() -> () {

    // signed and unsigned
    let _i8: i8 = 0i8;
    let _i16: i16 = 0i16;
    let _i32: i32 = 0i32;
    let _i64: i64 = 0i64;
    let _u8: u8 = 0u8;
    let _u16: u16 = 0u16;
    let _u32: u32 = 0u32;
    let _u64: u64 = 0u64;
    // arch dependent
    let i: isize = 0;
    let u: usize = 0;

    // bool
    let b0: bool = false;
    let b1: bool = true;
    let b = true; // type inference

    // char
    let c: char = 'x'; // utf32

    // float
    let _f32: f32 = 0f32;
    let _f64: f64 = 0f64;
    // let long double missing

    // sizes
    println!("i8  {}", std::mem::size_of::<i8>());
    println!("i16 {}", std::mem::size_of::<i16>());
    println!("i32 {}", std::mem::size_of::<i32>());
    println!("i64 {}", std::mem::size_of::<i64>());
    println!("u8  {}", std::mem::size_of::<u8>());
    println!("i16 {}", std::mem::size_of::<u16>());
    println!("u32 {}", std::mem::size_of::<u32>());
    println!("u64 {}", std::mem::size_of::<u64>());
    println!("isize {}", std::mem::size_of::<isize>());
    println!("usize {}", std::mem::size_of::<usize>());
    println!("bool {}", std::mem::size_of::<bool>());
    println!("f32 {}", std::mem::size_of::<f32>());
    println!("f64 {}", std::mem::size_of::<f64>());
    println!("char {}", std::mem::size_of::<char>());

    // literals
    let _: i8 = 0x00;
    let _: i8 = 0x00i8;
    let _: i8 = 0x00_i8;
    let _: i8 = 0b1100;
    let _: i8 = 0b1100i8;
    let _: i8 = 0b1100_i8;
    let _: i8 = 0o77;
    let _: i8 = 0o77i8;
    let _: i8 = 0o77_i8;

    let _: char = '\u{0000}';
    let _: char = 'µ'; // utf32
    let _: char = 'ǂ'; // utf32
}

fn struct_and_tuple() {
    struct X0 {
        x: u8,  // comma here optional
    };  // comma here optional
    struct X1 {
        x: (u8),    // not a tuple
        x1: (u8,),  // tuple with one element
        x2: (u8,u8),
        x3: (u8,u8,u8),
    }
    //
    struct S0;
    struct S1();
    struct S2{}
    //
    let x: (u8) = 0;
    let y: (u8,) = (0,);
    //
}

fn enum_and_discriminated_union() {
    struct S { x: u8, }     // externally defined struct
    enum E { X, Y, Z, }     // externally defined simple enum
    enum DX { DX0, DX1(), DX2{}, DX3(u8), DX4{x:u8}, DX5(E), DX6(S) }   // using inline and external defined types
}

fn array_and_slice() {
    // type aliasing
    type Q1 = u8;
    type Q2 = u8;
    type X10U8 = [u8;10];
    // Q1 == Q2
    let q1: Q1 = 0;
    let q2: Q2 = 0;
    if(q1 == q2) {
        println!("equal");
    }
    // array
    let arr1: X10U8;
    let arr2: X10U8 = [0,1,2,3,4,5,6,7,8,9];
    let arr3 = [10; 0];     // array with size 0
    let arr4 = [0; 10];
    let arr5 = [0; compile_time_const()];
    // let arr6 = [0; N];   // cannot use static
    let arr6 = [0; M];
    println!("arr3.len() = {}", arr3.len());
    println!("arr4.len() = {}", arr4.len());

    // slice
    let s: &[u8] = &[1;10];
    show(s);

    let msg = String::from("0123456789");
    let s0 = &msg[..];  // full slice
    let s1 = &msg[0..=2];   // 012
    let s2 = &msg[0..2];    // 01
    println!("{},{},{}", s0, s1, s2);
}

const fn compile_time_const() -> usize {
    10
}

static N: usize = 10;
// static N: usize = 10;   // static variables can not be redefined
// static N: usize;        // static must be initialized
// const M: usize;         // const must be initialized
const M: usize = 10;
// static const Q: usize = 10;  // qualifiers cannot be used together

fn show(xs: &[u8]) {
    println!("show");
    for x in xs {
        print!("{}", x);
    }
    println!();
}

fn loops() {
    // loop {}         // infinite loop
    let mut b: bool = true; // when using while true, compiler emits warning -> use loop {} instead
    while b {
        println!("while loop");
        b = false;
    }
    println!();
    for i in 0..10 {
        print!("{}, ", i);
    }
    println!();
    for i in 0..=10 {
        print!("{}, ", i);
    }
    println!();
    match 10 {
        0...5 => 0,
        _ => 1
    };

    // for loop with step
    for i in (0..10).step_by(2) {
        print!("{}, ", i);
    }
    println!();

    // reverse for loop with step
    for i in (0..10).rev().step_by(2) {
        print!("{}, ", i);
    }
    println!();

    // nested loop with break labels
    let mut cnt: u8 = 0;
    'outer: for _ in 0..10 {
            println!("outer");
    'inner: for _ in 0..10 {
            println!("inner");
            cnt += 1;
            if cnt % 3 == 0 {
                continue 'inner;
            }
            if cnt % 5 == 0 {
                break 'outer;
            }
        }
    }
}

fn ranges() {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).unwrap();
    // let x = inp.trim_end().parse::<u8>().unwrap_or(0);
    // let x = inp.trim_start().parse::<u8>().unwrap_or(0);
    let x = inp.trim().parse::<u8>().unwrap_or(0);  // trim start and end

    let r = match x {
        0x00 => 0,
        0x01...0x0F => 1,   // excluding range
        0x0F..=0x1F => 2,   // including range
        _ => 3,
    };
    println!("{}", r);
}

fn main() {
    enum_and_discriminated_union();
    types_and_literals();
    struct_and_tuple();
    array_and_slice();
    ranges();
    loops();
}

