#![allow(unused)]

#![feature(vec_remove_item)]
/// using nightly allows to use unstable feature
/// the features still have to be enabled

// #[rustfmt::skip]
fn  foo ( ) {
}

use std::collections::HashMap;

fn using_hashmap() {
    let mut hm: HashMap<u32, u32>;
    hm = HashMap::new();
}

fn using_vector() {
    let mut v: Vec<i8> = Vec::new();
    println!("{}, {}", v.len(), v.capacity());
    for i in -100..=100 {
        v.push(i);
        // println!("{}, {}", v.len(), v.capacity());
    }

    // remove
    let mut v: Vec<i8> = Vec::new();
    // v.remove(0);   // panic
    println!("{}, {}", v.len(), v.capacity());
    println!("{:?}", v);

    // remove_item
    let mut v: Vec<u8> = Vec::new();
    v.remove_item(&0);   // no panic
    v.remove_item(&1);
    v.remove_item(&3);
    v.remove_item(&4);
    v.remove_item(&5);
    v.push(10);
    v.push(11);
    v.push(12);
    v.push(13);
    v.push(14);
    v.push(15);
    v.remove_item(&10);
    println!("remote_item(10) {:?}", v);

    // append
    let mut v1 = vec![1, 2, 3, 4, 5];
    let mut v2 = vec![6, 7, 8, 9, 10];
    v1.append(&mut v2);
    v2.push(1);
    v1.push(100);
    if let Some(v) = v1.pop() {
        println!("pop {}", v);
    }

    // type annotation needed
    let mut v: Vec<u8> = vec![];
    if let Some(x) = v.pop() {
        println!("pop {}", x);
    } else {
        println!("empty");
    }

    // with_capacity
    let mut v: Vec<i8> = Vec::with_capacity(100);
    println!("{}, {}", v.len(), v.capacity());

    // extend
    let mut v1: Vec<u8> = vec![1,2,3];
    let mut v2: Vec<u8> = vec![4,5,6];
    // v1.extend(v2);
    v1.extend(&v2);
    println!("extend {:?}", v1);
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

    // let v = (0..10).collect::<[u8; 10]>();
    // let v = (0..10).collect::<[u8]>();
    let mut v = (0..10).collect::<Vec<_>>();
    v.push(1i32);
}

struct S {}

// size_of = 104
// #[derive(Debug)]
enum E {
    E0(u8),
    E1(u16),
    E2(u32),
    E3(u64),
    E4(u128),
    E5([u8; 100]),
    E6(f64),
}

fn mem_size_of() {
    println!("size_of<u8>={}", std::mem::size_of::<u8>());
    println!("size_of<u16>={}", std::mem::size_of::<u16>());
    println!("size_of<u32>={}", std::mem::size_of::<u32>());
    println!("size_of<u64>={}", std::mem::size_of::<u64>());
    println!("size_of<u128>={}", std::mem::size_of::<u128>());
    println!("size_of<f32>={}", std::mem::size_of::<f32>());
    println!("size_of<f64>={}", std::mem::size_of::<f64>());
    // struct
    println!("size_of<S>={}", std::mem::size_of::<S>());
    println!("size_of<E>={}", std::mem::size_of::<E>());
    let ps: &S = &S {};
    let pe: &E = &E::E5([0; 100]);
    let pe: &E = &mut E::E5([0; 100]);
    if let E::E5(mut x) = *pe {
        for i in 0..100 {
            x[i] = i as u8;
        }
    }

    println!("{:?}", [0; 32]); // fmt:Debug implemented
    // println!("{:?}", [0; 33]); // fmt::Debug not implemented

    match *pe {
        E::E5(x) => {
            for i in x.iter() {
                print!("{},", i);
            }
        }
        _ => (),
    }
}

fn test(v: &isize) {
    assert_eq!(v, &11111);
    {
        let v1 = &11111;
        assert_eq!(v1, v);
    }

    let b = Box::new(&11111);
    assert_eq!(v, *b);

}

fn main() {
    using_vector();
    mem_size_of();

    let v1 = &11111;
    let v2 = &11111;
    assert_eq!(v1, v2);
    {
        let v3 = &11111 ;
        assert_eq!(v1, v3);
    }
    test(v1);
}

