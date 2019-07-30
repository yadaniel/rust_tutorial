// #![deny(unsafe_code)]
#![allow(unused)]
#![macro_use]
#![no_main]
#![no_std]

// The cortex-m-rt crate is a very small runtime that boots the device, initializes RAM and then calls main. 
// It does all this implicitly; you only need to link to it with extern crate to opt into this runtime. 
// The cortex-m crate provides an API to use functionality common to all Cortex-M microcontrollers.

extern crate cortex_m;
extern crate cortex_m_rt;

use cortex_m::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn default_handler(info: &PanicInfo) -> ! {
    loop {}
}

// As we are not using interrupts, we just register a dummy catch all handler
#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler_; 240];

extern "C" fn default_handler_() {
    asm::bkpt();
}

// use clang layout
#[repr(C)]
struct SysTick {
    pub csr: u32,
    pub rvr: u32,
    pub cvr: u32,
    pub calib: u32,
}

fn get_cvr() -> u32 {
    let systick = unsafe { &mut *(0xE000_E010 as *mut SysTick) };
    let time = unsafe { core::ptr::read_volatile(&mut systick.cvr) };
    time
}

// use cortex_m::volatile_register::{RW, RO};
use cortex_m::{*};
use cortex_m_rt::{*};   // provides macro entry

#[entry]
fn main_entry() -> ! {
    loop {
        //
    }
}

