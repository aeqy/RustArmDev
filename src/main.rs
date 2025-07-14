#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    loop {}
}