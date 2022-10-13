#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
#![feature(linkage)]
#![feature(asm_sym)]

pub mod serial;

use core::arch::asm;
use core::panic::PanicInfo;

const STACK_SIZE: usize = 4096 * 5;


struct Stack {
    _data: [u8; STACK_SIZE]
}


static STACK: Stack = Stack {
    _data: [0; STACK_SIZE]
};


#[naked]
#[export_name = "_start"]
#[link_section = ".text.entry"]
pub extern "C" fn _start() {
    unsafe {
        asm!(
            "la sp, {stack}",
            "call {runo_main}",
            runo_main = sym runo_main,
            stack = sym STACK,
            options(noreturn)
        );
    }
}


#[no_mangle]
#[linkage = "weak"]
fn kmain() -> i32 {
    panic!("Can not find main function!");
}


#[no_mangle]
fn exit(_n: i32) -> ! {
    system_exit();
}


#[no_mangle]
pub extern "C" fn runo_main() -> ! {
    clear_bss();
    exit(kmain());
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


fn clear_bss() {
    extern "C" {
        pub fn sbss();
        pub fn ebss();
    }

    (sbss as usize..ebss as usize)
        .for_each(|a| {
            unsafe {
                (a as *mut u8).write_volatile(0);
            }
        })
}


fn system_exit() -> ! {
    // let (error, value): (usize, usize);
    unsafe{
        asm!(
            "ecall",
            in("a7") 0x53525354 as usize,
            in("a6") 0x0 as usize,
            inlateout("a0") 0x0 as usize => _,
            inlateout("a1") 0x0 as usize => _,
        );
    }
    panic!("Should be exit");
    // unreachable!();
}
