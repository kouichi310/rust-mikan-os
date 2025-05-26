#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    // println!("{}", _panic);
    loop {}
}

#[unsafe(no_mangle)]
#[allow(unreachable_code)]
pub extern "C" fn kernel_main() {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
