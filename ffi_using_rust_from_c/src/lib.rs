#![allow(unused)]

// $ rustup toolchain list
// stable-i686-pc-windows-gnu (default)
// stable-x86_64-pc-windows-gnu
// nightly-i686-pc-windows-gnu
// nightly-x86_64-pc-windows-gnu

/// tutorial part

#[repr(C)]
struct X {
    x0: u8,
    x1: u16,
    x2: u32,
    x3: u64
}

#[no_mangle]
extern "C" fn called_by_c(x:u8, y:u8) -> u8 {
    x+y
}

#[no_mangle]
extern "C" fn print_x(x: &X) {
    println!("x0={}", (*x).x0);
    println!("x1={}", (*x).x1);
    println!("x2={}", (*x).x2);
    println!("x3={}", (*x).x3);
}

/// tests

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

