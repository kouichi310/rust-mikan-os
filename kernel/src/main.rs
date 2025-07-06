#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    // println!("{}", _panic);
    loop {}
}

/// # Safety
///
/// - `frame_buffer_base` は `frame_buffer_size * 8` バイト分の有効なメモリ領域を指している必要があります。
/// - この関数は UEFI ブートローダから正しく初期化された状態で呼び出される前提です。
#[unsafe(no_mangle)]
#[allow(unreachable_code)]
pub unsafe extern "C" fn kernel_main(frame_buffer_base: *mut u64, frame_buffer_size: u64) -> ! {
    for i in 0..frame_buffer_size {
        unsafe {
            *frame_buffer_base.add(i as usize) = i % 256;
        }
    }
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
