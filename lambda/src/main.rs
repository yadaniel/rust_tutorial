#![allow(unused)]

use std::ops::Fn;

fn apply(x: i32, f: fn(i32)->i32) -> i32 {
    f(x)
}

fn cube(x: i32) -> i32 {
    x*x*x
}

fn retfunc() -> (fn(i32)->i32) {
    cube
}

//fn retlambda() -> (|i32|->i32) {
//    |x| x
//}


fn factory() -> Box<Fn(i32)->i32> {
    let num = 5;
    Box::new(move |x| x + num)
}

fn main() {
    let x1 = apply(2, |x|x+1);
    let x2 = apply(2, |x|x*2);

    fn add_double(x: i32) -> i32 {
        (x+1)*2
    }
    let x3 = add_double(1);

    let q1 : fn(i32) -> i32 = add_double;
    let q2 : fn(i32) -> i32 = cube;
    let q3 = add_double;
    let q4 = cube;

    let id1  = |x:i32|x;
    //let id2 : |i32|->i32 = |x|x;
    //let id2 : Fn(i32)->i32 = |x|x;
}

