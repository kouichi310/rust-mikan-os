#![no_main]
#![no_std]

use core::ptr::{self, null};
use core::{panic::PanicInfo, ptr::{null_mut}};
use uefi::system_table::EfiSystemTable;

mod uefi;
use uefi::{boot_services::EfiBootServices, file_systems::{EfiFileProtocol, EfiLoadedImageProtocol, EfiSimpleFileSystemProtocol, EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL}, guids::{EFI_LOADED_IMAGE_PROTOCOL_GUID, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID}, status::EfiStatus, types::EfiHandle};
use utils::print::{set_con_out};

#[macro_use]
mod utils;


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

fn open_root_dir(image_handle: EfiHandle, root: &mut *mut EfiFileProtocol, bs: &EfiBootServices) -> EfiStatus {
    let null_handle = EfiHandle(null_mut());


    let loaded_image = bs.open_protocol::<EfiLoadedImageProtocol>(
        image_handle,
        &EFI_LOADED_IMAGE_PROTOCOL_GUID,
        image_handle,
        null_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    ).unwrap();
    println!("Loaded Image Protocol opened: {:?}", loaded_image);

    let fs = bs.open_protocol::<EfiSimpleFileSystemProtocol>(
        unsafe{(*loaded_image).device_handle},
        &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
        image_handle,
        null_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    ).unwrap();

    println!("Simple File System Protocol opened: {:?}", fs);

    unsafe{(*fs).open_volume(root)};

    EfiStatus::Success
}

#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(image_handle: EfiHandle, system_table: &'static EfiSystemTable) -> EfiStatus {
    set_con_out(system_table.con_out());
    system_table.con_out().reset(true);
    println!("Hello, UEFI World!");

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
        println!("Failed to get memory map");
        return status;
    }

    println!("Memory map acquired.");

    let root_dir: &mut *mut EfiFileProtocol = &mut ptr::null_mut();
    open_root_dir(image_handle, root_dir, system_table.boot_services());

    println!("Directory opened.");

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
