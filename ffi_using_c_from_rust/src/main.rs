#![allow(unused)]

extern crate libc;
use libc::{wchar_t, c_char, c_int, size_t};
use std::ffi::CString;

#[repr(C)]
struct CData {
    x: u8,
    y: u16,
    z: u32
}

// // #[link(name = "foobar", kind = "static")]
// #[link(name = "foobar")]
// extern "C" {
//     fn cdata_print(cdata: CData);
// }

// fn call_c() {
//     let cdata = CData {x:0, y:1, z:0};
//     unsafe { cdata_print(cdata); }
// }

extern "C" {
    fn strlen(s: *const u8) -> size_t;
    // fn wcslen(s: *const wchar_t) -> size_t;
}

fn main() {
    // call_c();

    unsafe {
        let c_str = CString::new(b"12345" as &[u8]).unwrap();
        println!("libc::strlen(\"12345\")={}", libc::strlen(c_str.as_ptr()));

        // not correct
        println!("strlen(\"1\")={}", strlen("1".as_ptr()));
        println!("strlen(\"123\")={}", strlen("123".as_ptr()));
        println!("strlen(\"12345\")={}", strlen("12345".as_ptr()));
        println!("strlen(\"123456789\")={}", strlen("123456789".as_ptr()));

        let c_str = CString::new(b"12345" as &[u8]).unwrap();
        println!("libc::strlen(\"12345\")={}", libc::strlen(c_str.as_ptr()));
    }
}



