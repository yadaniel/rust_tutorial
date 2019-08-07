#![allow(unused)]

#[derive(Debug)]
pub struct Foo {
    x: u8,
    y: u8,
    z: u8
}

impl Foo {
    pub fn new(x:u8, y:u8, z:u8) -> Foo {
        Foo {x:x, y:y, z:z}
    }
}

