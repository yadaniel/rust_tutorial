#![allow(unused)]
#![no_main]
#![no_std]

/// cortex_m:
extern crate cortex_m;
use cortex_m::asm;

/// cortex_m_rt: memory layout, vector table
/// expects memory.x file
extern crate cortex_m_rt;
use cortex_m_rt::entry;

///stm32l432xx_hal: 
extern crate stm32l432xx_hal as hal;
// use crate::hal::{prelude::*, stm32};

#[panic_handler]
fn default_handler(info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// use core::panic::PanicInfo;
// use core::sync::atomic::{self, Ordering};
// #[inline(never)]
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {
//         atomic::compiler_fence(Ordering::SeqCst);
//     }
// }

#[no_mangle]
#[link_section = ".text"]
fn foo(x: u8) -> u16 {
    let tmp: u16 = x as u16;
    tmp | (x as u16)
}

#[no_mangle]
#[link_section = ".data"]
static BAR: u8 = 10;

extern {
    fn foo_ext();
    static BAREXT: u8;
}

#[entry]
#[no_mangle]
#[export_name = "main"]
#[link_section = ".text"]
fn main() -> ! {
    loop {
        //
    }
}

