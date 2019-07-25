#![allow(unused)]

fn f1() {
    print!("f1 ")
}

fn f2() {
    print!("f2 ")
}

fn f3() {
    print!("f3 ")
}

fn use_fs() {
    // cargo fmt appends -> () to fn() -> ()
    // let fs: [fn() -> () -> (); 3] = [f1, f2, f3];
    // let fs: [fn() -> (); 3] = [f1, f2, f3];
    let fs: [fn(); 3] = [f1, f2, f3];

    for f in fs.iter() {
        f()
    }
}

/// bottom type
fn foo1() -> ! {
    loop {}
}

fn foo2() -> () {
    loop {}
}

fn foo3() -> u32 {
    extern crate rand;
    rand::random::<u32>()
}

fn use_foo() {
    extern crate rand;
    use std::thread;
    loop {
        println!("...");
        thread::sleep(std::time::Duration::new(1, 0));
        let x: u32 = match rand::random::<u32>() % 100 {
            1 => continue,
            2 => continue,
            3 => foo1(),
            4 => 4,
            x => x,
            _ => 5,
        };
        println!("{}", x);
    }
}

fn main() {
    use_fs();
    use_foo()
}
