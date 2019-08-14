#![allow(unused)]

use std::collections::HashMap;

fn using_hashmap() {
    let mut hm: HashMap<u32, u32>;
    hm = HashMap::new();
}

fn using_vector() {
    let mut v: Vec<u32> = Vec::new();
}

fn using_array_slice() {
    let mut arr: [u32; 5] = [0, 1, 2, 3, 4];
    arr.iter().map(|x| x + 1);

    let v: Vec<u8> = (0..10).collect();
    let v: Vec<u8> = (0u8..10).collect();
    let v: Vec<u8> = (0..10u8).collect();
    let v: Vec<u8> = (0u8..10u8).collect();
    let v = (0u8..10).collect::<Vec<u8>>();
    let v = (0u8..10).collect::<Vec<u8>>().iter().map(|x| x + 1);
}

fn main() {
    println!("Hello, world!");
}
