#![allow(unused)]
#![deny(unsafe_code)]

pub struct S<T1, T2> {
    t1: T1,
    t2: T2,
}

impl<T1, T2> S<T1, T2> {
    fn foo(&self) {
        println!("does not depend on T1, T2");
    }

    fn bar<T3>(&self, t3: T3) -> T3 {
        t3
    }
}

#[rustfmt::skip]
fn main() {
    // type annotation on value
    let s = S { t1: 1u8, t2: 1u32 };
    s.foo();
    // type annotation at variable
    let s: S<u8,u32> = S { t1: 1, t2: 1 };
    s.foo();

    // s.foo::<u8,u16>();   // this is wrong, foo is not template
    let q = s.bar::<u16>(1u16);   // this is ok, bar is template
    println!("{}", q);

    // usage with Vec
    let v: Vec<u8>  = (0..=10).collect();
    let v = (0..=10).collect::<Vec<u8>>();

    // collect is part of std::iter::Iterator trait with signature 
    // fn collect<B>(self) -> B 
    // so collect is template function
}
