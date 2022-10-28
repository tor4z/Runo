#![allow(dead_code)]
#![no_std]
#![no_main]

#[no_mangle]
fn kmain() -> i32 {
    plat::serial::serial_putchar('Y' as u8);
    app::main()
}
