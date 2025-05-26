#![no_main]
#![no_std]

use core::fmt::Write;
use core::{panic::PanicInfo, ptr::null_mut};
use uefi::memory::EfiMemoryDescriptor;
use uefi::system_table::EfiSystemTable;

mod uefi;
use uefi::types::{EfiFileAttribute, EfiFileOpenMode};
use uefi::{
    boot_services::EfiBootServices,
    file_systems::{
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL, EfiFileProtocol, EfiLoadedImageProtocol,
        EfiSimpleFileSystemProtocol,
    },
    guids::{EFI_LOADED_IMAGE_PROTOCOL_GUID, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID},
    status::EfiStatus,
    types::EfiHandle,
};
use utils::fixed_buffer::FixedBuffer;
use utils::print::set_con_out;

#[macro_use]
mod utils;

struct MemoryMap<'a> {
    buffer: &'a mut [u8],
    map_size: usize,
    map_key: usize,
    descriptor_size: usize,
    descriptor_version: u32,
}

fn get_memory_map_unicode(memory_type: u32) -> &'static str {
    match memory_type {
        0 => "Reserved Memory Type",
        1 => "Loader Code",
        2 => "Loader Data",
        3 => "Boot Services Code",
        4 => "Boot Services Data",
        5 => "Runtime Services Code",
        6 => "Runtime Services Data",
        7 => "Conventional Memory",
        8 => "Unusable Memory",
        9 => "ACPI Reclaim Memory",
        10 => "ACPI Memory NVS",
        11 => "Memory Mapped I/O",
        12 => "Memory Mapped I/O Port Space",
        13 => "Pal Code",
        14 => "Persistent Memory",
        15 => "Unassigned Memory Type",
        16 => "Max Memory Type",
        _ => "Unknown Memory Type",
    }
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

fn save_memory_map(
    map: &[u8],
    file: &EfiFileProtocol,
    descriptor_size: usize,
    map_size: usize,
) -> EfiStatus {
    let header = "Index,\t\tType,\tType(name),\tPhysicalStart,\tNumberOfPages,\tAttribute\n";
    let len = header.len();

    let written_size = file.write(len, header.as_ptr()).unwrap();
    if written_size != len {
        println!("Failed to write header to file");
        return EfiStatus::Success;
    }

    let mut index = 0;
    let mut offset = 0;

    while offset < map_size {
        let memory_descriptor = unsafe {
            (map.as_ptr().add(offset) as *const EfiMemoryDescriptor)
                .as_ref()
                .unwrap()
        };

        let mut buf = [0u8; 128];
        let mut fb = FixedBuffer::new(&mut buf);

        writeln!(
            fb,
            "{:<5} {:<6} {:<25} {:<#16x} {:<#12x} {:<#18x}",
            index,
            memory_descriptor.memory_type,
            get_memory_map_unicode(memory_descriptor.memory_type),
            memory_descriptor.physical_start,
            memory_descriptor.number_of_pages,
            memory_descriptor.attribute,
        )
        .unwrap();

        let _res = file
            .write(fb.as_bytes().len(), fb.as_bytes().as_ptr())
            .unwrap();
        if _res != fb.as_bytes().len() {
            println!("Failed to write memory descriptor to file");
            return EfiStatus::Success;
        }

        index += 1;
        offset += descriptor_size;
    }

    file.close().unwrap();
    println!("Memory map saved successfully.");

    EfiStatus::Success
}

fn open_root_dir(
    image_handle: EfiHandle,
    bs: &EfiBootServices,
) -> Result<&EfiFileProtocol, EfiStatus> {
    let null_handle = EfiHandle(null_mut());

    let loaded_image = match bs.open_protocol::<EfiLoadedImageProtocol>(
        image_handle,
        &EFI_LOADED_IMAGE_PROTOCOL_GUID,
        image_handle,
        null_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    ) {
        Ok(image) => unsafe { (image as *const EfiLoadedImageProtocol).as_ref().unwrap() },
        Err(status) => {
            println!("Failed to open Loaded Image Protocol: {:?}", status);
            return Err(status);
        }
    };

    println!("Loaded Image Protocol opened successfully");

    let fs = match bs.open_protocol::<EfiSimpleFileSystemProtocol>(
        loaded_image.device_handle,
        &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
        image_handle,
        null_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    ) {
        Ok(fs) => unsafe { fs.as_mut().unwrap() },
        Err(status) => {
            println!("Failed to open Simple File System Protocol: {:?}", status);
            return Err(status);
        }
    };

    println!("Simple File System Protocol opened successfully");

    fs.open_volume()
}

#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(
    image_handle: EfiHandle,
    system_table: &'static EfiSystemTable,
) -> EfiStatus {
    set_con_out(system_table.con_out());
    system_table.con_out().reset(true);
    println!("Hello, UEFI World!");

    let mut buf = [0u8; 4096 * 4];
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

    let efi_file_proto = open_root_dir(image_handle, system_table.boot_services()).unwrap();
    println!("Root directory opened successfully");

    let opened_handle = efi_file_proto
        .open(
            "\\memmap.csv",
            EfiFileOpenMode::CreateReadWrite,
            EfiFileAttribute::None,
        )
        .unwrap();

    save_memory_map(
        memory_map.buffer,
        opened_handle,
        memory_map.descriptor_size,
        memory_map.map_size,
    );

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
