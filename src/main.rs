#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
    
}
