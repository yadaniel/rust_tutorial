#![allow(unused)]

extern crate rand as random;
use random::Rng as rgen;

#[derive(Copy, Clone)]
struct E {
    idx: Option<u8>,
    val: Option<u32>
}

fn init(arr: &mut[[E;10];10]) {
    let mut rng = random::thread_rng();
    for i in 0..10 {
        for j in 0..10 {
            match rng.gen_range(0, 100) {
                0..=75 => continue,
                _ => arr[i][j] = E {idx:None, val:Some(rng.gen_range(0, 100))}
            }
        }
    }
}

fn print(arr: &[[E;10];10]) {
    for i in 0..10 {
        for j in 0..10 {
            match arr[i][j] {
                E {idx:_, val:Some(value)} => print!("{0:3}, ", value),
                E {idx:_, val:None} => (),
            }
        }
        println!();
    }
        println!();
}

fn print_(arr: &[[E;10];10]) {
    for i in 0..10 {
        for j in 0..10 {
            match arr[i][j] {
                // E {idx:_, val:Some(value)} if value == 0 => print!("   , "),
                // E {idx:_, val:Some(value)} if value != 0 => print!("{0:3}, ", value),
                E {idx:_, val:Some(0)} => print!("   , "),
                E {idx:_, val:Some(value)} => print!("{0:3}, ", value),
                _ => (),
            }
        }
        println!();
    }
        println!();
}

fn run() {
    let mut arr: [[E;10];10] = [[E{idx:None, val:Some(0)};10]; 10];
    print(&arr);
    init(&mut arr);
    // print(&arr);
    print_(&arr);
}

#[test]
fn test0() {
}

#[test]
fn test1() {
}

// impl std::marker::Copy for std::boxed::Box<u8> {
// }

fn main() {
    run();

    // raw pointers
    let x: u8 = 0;
    let px: *const u8 = &x;
    let px: *const u8 = &0;
    // px = &1; // px is const
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    let mut px: *mut u8 = &mut x;
    let mut py: *mut u8 = &mut y;
    px = &mut y;
    py = &mut x;
    x = 1;
    y = 10;
    // no unsafe needed
    println!("x={}", x);
    println!("y={}", y);
    // unsafe needed when read access by pointer
    unsafe {
        println!("x=*py={}={}", x, *py);
        println!("y=*px={}={}", y, *px);
    }
    // unsafe needed when write access by pointer
    unsafe { *px = 100; }
    unsafe { *py = 100; }

    //
    let mut x: u8 = 0;
    let mut rx: Box<u8> = Box::new(x);
    println!("{},{}", x, rx);
    x = 1;
    println!("{},{}", x, rx);
    *rx = 10;
    println!("{},{}", x, rx);
    //
    // let f: [Box<u8>; 10] = [Box::new(0); 10];
    let b1 = Box::new(10);
    let b2 = *b1;   // just content
    let b3 = *b1;   // just content
    let b4 = b1;   // move
    // println!("{}={}", b1, *b1);   // can not use after move
    println!("{}", b2);
    println!("{}", b3);
    println!("{}={}", b4, *b4);
    //
    use std::cell::RefCell;
    let c: RefCell<u8> = RefCell::<u8>::new(100);
    // let mut c: RefCell<u8> = RefCell::<u8>::new(100);
    println!("{:?}", c);
    c.replace(200);
    println!("{:?}", c);
    // let cc = &c;
    let mut cc = &c;
    println!("{:?} == {:?}", c, cc);
    cc.replace(10);
    println!("{:?} == {:?}", c, cc);
    cc.replace(10);
    c.replace(1);
    println!("{:?} == {:?}", c, cc);
    //
    let val = RefCell::<u8>::new(121);
    let arr: [&RefCell<u8>;10] = [&val; 10];
    println!("{:?}", arr[0]);
    println!("{:?}", arr[1]);
    println!("{:?}", arr[3]);
    println!("{:?}", arr[4]);
    arr[3].replace(212);
    println!("{:?}", arr[0]);
    println!("{:?}", arr[1]);
    println!("{:?}", arr[3]);
    println!("{:?}", arr[4]);

}
