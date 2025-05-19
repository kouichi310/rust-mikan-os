#![no_main]
#![no_std]

use core::ptr;
use core::{ffi::c_void, panic::PanicInfo, ptr::{null_mut}};
use uefi::system_table::EfiSystemTable;
use utf16_literal::utf16;

mod uefi;
use uefi::{boot_services::EfiBootServices, file_systems::{EfiFileProtocol, EfiLoadedImageProtocol, EfiSimpleFileSystemProtocol, EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL}, guids::{EFI_LOADED_IMAGE_PROTOCOL_GUID, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID}, status::EfiStatus, types::EfiHandle};

struct MemoryMap<'a> {
    buffer: &'a mut [u8],
    map_size: usize,
    map_key: usize,
    descriptor_size: usize,
    descriptor_version: u32,
}

fn get_memory_map(memory_map: &mut MemoryMap<'_>, bs: &EfiBootServices<'static>) -> EfiStatus {
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

fn open_root_dir(image_handle: EfiHandle, root: &mut *mut EfiFileProtocol, bs: &EfiBootServices<'static>) -> EfiStatus {
    let mut loaded_image: *mut EfiLoadedImageProtocol = null_mut();
    let mut fs: *mut EfiSimpleFileSystemProtocol = null_mut();
    let null_handle = EfiHandle(null_mut());


    bs.open_protocol(
        image_handle,
        &EFI_LOADED_IMAGE_PROTOCOL_GUID,
        (&mut loaded_image as *mut *mut EfiLoadedImageProtocol) as *mut *mut c_void,
        image_handle,
        null_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    );

    bs.open_protocol(
        unsafe{(*loaded_image).device_handle},
        &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
        (&mut fs as *mut *mut EfiSimpleFileSystemProtocol) as *mut *mut c_void,
        image_handle,
        null_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    );

    unsafe{(*fs).open_volume(root)};

    EfiStatus::Success
}

#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(image_handle: EfiHandle, system_table: &EfiSystemTable<'static>) -> EfiStatus {
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

    let root_dir: &mut *mut EfiFileProtocol = &mut ptr::null_mut();
    open_root_dir(image_handle, root_dir, system_table.boot_services());

    con_out.output_string(utf16!("Directory opened.\r\n").as_ptr());

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
