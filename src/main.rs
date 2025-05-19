#![no_main]
#![no_std]

mod uefi;

use core::panic::PanicInfo;
use core::ffi::c_void;
use uefi::{SystemTable, output::print};
use utf16_literal::utf16;

#[unsafe(no_mangle)]
pub extern "efiapi" fn kernel_main(_image: *mut c_void, system_table: *mut SystemTable) -> usize {
    let st = unsafe { &*system_table };
    print(unsafe { st.con_out() }, utf16!("Hello, world!\n"));
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
