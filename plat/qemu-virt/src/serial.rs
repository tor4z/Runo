use spin::Mutex;
use uart_16550::MmioSerialPort;
use lazy_static::lazy_static;


lazy_static! {
    pub static ref SERIAL: Mutex<MmioSerialPort> = {
        Mutex::new(unsafe { MmioSerialPort::new(0x1000_0000) })
    };
}


pub fn serial_putchar(c: u8) {
    SERIAL.lock().send(c);
    // SERIAL.wait().expect("Serail port busy!").lock().send(c);
}

