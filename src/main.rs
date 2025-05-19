#![no_main]
#![no_std]

use core::panic::PanicInfo;
use utf16_literal::utf16;

mod uefi;
use uefi::{boot_services::EfiBootServices, status::EfiStatus, types::EfiHandle, SystemTable};

struct MemoryMap<'a> {
    buffer: &'a mut [u8],
    map_size: usize,
    map_key: usize,
    descriptor_size: usize,
    descriptor_version: u32,
}

fn get_memory_map(memory_map: &mut MemoryMap<'_>, bs: &EfiBootServices) -> EfiStatus {
    match bs.get_memory_map(memory_map.buffer) {
        Ok((map_size, map_key, descriptor_size, descriptor_version)) => {
            memory_map.map_size = map_size;
            memory_map.map_key = map_key;
            memory_map.descriptor_size = descriptor_size;
            memory_map.descriptor_version = descriptor_version;
            EfiStatus::Success
        }
        Err(status) => status,
    }
}

#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(_image_handle: EfiHandle, system_table: &SystemTable) -> EfiStatus {
    let con_out = system_table.con_out();
    con_out.reset(true);
    con_out.output_string(utf16!("Hello, Rust UEFI\r\n").as_ptr());

    let mut buf = [0u8; 4096*4];
    let mut memory_map = MemoryMap {
        buffer: &mut buf,
        map_size: 0,
        map_key: 0,
        descriptor_size: 0,
        descriptor_version: 0,
    };

    let status = get_memory_map(&mut memory_map, system_table.boot_services());
    if status != EfiStatus::Success {
        con_out.output_string(utf16!("Failed to get memory map.\r\n").as_ptr());
        return status;
    }

    con_out.output_string(utf16!("Memory map acquired.\r\n").as_ptr());

    loop {}

    uefi::status::EfiStatus::Success
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
    
}
