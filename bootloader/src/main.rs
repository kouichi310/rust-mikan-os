//! Refactored UEFI bootloader `main.rs`
#![no_main]
#![no_std]

use core::{arch::asm, fmt::Write, panic::PanicInfo};
use uefi::status::EfiStatus;
use uefi::system_table::EfiSystemTable;

mod uefi;

#[macro_use]
mod utils;

use uefi::{
    boot_services::EfiBootServices,
    file_systems::{
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL, EfiFileProtocol, EfiLoadedImageProtocol,
        EfiSimpleFileSystemProtocol,
    },
    guids::{EFI_LOADED_IMAGE_PROTOCOL_GUID, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID},
    types::{EfiAllocateType, EfiHandle, EfiMemoryType},
};
use utils::{fixed_buffer::FixedBuffer, print::setup_console};

const KERNEL_BASE_ADDR: u64 = 0x0010_0000;

/// Wrapper for memory map buffer and metadata
struct MemoryMap<'a> {
    buf: &'a mut [u8],
    map_size: usize,
    map_key: usize,
    desc_size: usize,
    desc_version: u32,
}

impl<'a> MemoryMap<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self {
            buf: buffer,
            map_size: 0,
            map_key: 0,
            desc_size: 0,
            desc_version: 0,
        }
    }

    pub fn acquire(&mut self, bs: &EfiBootServices) -> Result<(), EfiStatus> {
        let (size, key, desc_size, desc_ver) = bs.get_memory_map(self.buf)?;
        self.map_size = size;
        self.map_key = key;
        self.desc_size = desc_size;
        self.desc_version = desc_ver;
        Ok(())
    }
}

fn get_memory_type_name(memory_type: u32) -> &'static str {
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

/// Save memory map to a CSV file
fn save_memory_map(memmap: &MemoryMap, file: &EfiFileProtocol) -> Result<(), EfiStatus> {
    let header = "Index,\t\tType,\tType(name),\tPhysicalStart,\tNumberOfPages,\tAttribute\n";
    let len = header.len();
    let written_size = file.write(len, header.as_ptr()).unwrap();
    if written_size != len {
        uefi_println!("Failed to write header to file");
        return Err(EfiStatus::EfiLoadError);
    }

    let mut index = 0;
    let mut offset = 0;
    while offset < memmap.map_size {
        let desc = unsafe {
            (memmap.buf.as_ptr().add(offset) as *const uefi::memory::EfiMemoryDescriptor)
                .as_ref()
                .unwrap()
        };
        let mut buf = [0u8; 128];
        let mut fb = FixedBuffer::new(&mut buf);
        writeln!(
            fb,
            "{:<5} {:<6} {:<25} {:#016x} {:#012x} {:#018x}",
            index,
            desc.memory_type,
            get_memory_type_name(desc.memory_type),
            desc.physical_start,
            desc.number_of_pages,
            desc.attribute,
        )
        .unwrap();

        let _res = file
            .write(fb.as_bytes().len(), fb.as_bytes().as_ptr())
            .unwrap();
        if _res != fb.as_bytes().len() {
            uefi_println!("Failed to write memory descriptor to file");
            return Err(EfiStatus::EfiLoadError);
        }

        index += 1;
        offset += memmap.desc_size;
    }
    file.close().ok();
    uefi_println!("Memory map saved to file successfully");
    Ok(())
}

/// Open the root directory of the current image
fn open_root_dir(
    image_handle: EfiHandle,
    bs: &EfiBootServices,
) -> Result<&EfiFileProtocol, EfiStatus> {
    let null_handle = EfiHandle(core::ptr::null_mut());

    let loaded = bs.open_protocol::<EfiLoadedImageProtocol>(
        image_handle,
        &EFI_LOADED_IMAGE_PROTOCOL_GUID,
        image_handle,
        null_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    )?;
    let loaded = unsafe { loaded.as_ref().unwrap() };

    let fs_ptr = bs.open_protocol::<EfiSimpleFileSystemProtocol>(
        loaded.device_handle,
        &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
        image_handle,
        null_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    )?;
    let fs = unsafe { fs_ptr.as_mut().unwrap() };
    fs.open_volume()
}

/// Load kernel binary and return its entry point function pointer
fn load_kernel(root: &EfiFileProtocol, bs: &EfiBootServices) -> Result<fn(), EfiStatus> {
    let kernel = root.open(
        "\\rust_mikan_os_kernel",
        uefi::types::EfiFileOpenMode::Read,
        uefi::types::EfiFileAttribute::None,
    )?;
    let info = kernel.get_info()?;
    let size = info.file_size as usize;

    bs.allocate_pages(
        EfiAllocateType::AllocateAddress,
        EfiMemoryType::EfiLoaderData,
        size,
        KERNEL_BASE_ADDR,
    )?;

    kernel.read(size, KERNEL_BASE_ADDR)?;

    const KERNEL_ENTRY_OFFSET: usize = 24;
    let entry = unsafe { *((KERNEL_BASE_ADDR + KERNEL_ENTRY_OFFSET) as *const u64) };
    Ok(unsafe { core::mem::transmute::<*const (), fn()>(entry as usize as *const ()) })
}

/// Exit boot services and jump to kernel entry
fn exit_and_jump(bs: &EfiBootServices, image_handle: EfiHandle, map_key: usize, entry: fn()) -> ! {
    let _ = bs.exit_boot_service(image_handle, map_key);
    uefi_println!("Exiting boot services and jumping to kernel...");
    entry();
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(
    image_handle: EfiHandle,
    system_table: &'static EfiSystemTable,
) -> EfiStatus {
    setup_console(system_table);
    uefi_println!("Starting UEFI bootloader");

    let bs = system_table.boot_services();

    let mut mem_buf = [0u8; 4096 * 4];
    let mut memmap = MemoryMap::new(&mut mem_buf);
    if memmap.acquire(bs).is_err() {
        uefi_println!("Failed to acquire memory map");
        return EfiStatus::EfiLoadError;
    }
    uefi_println!("Memory map acquired");

    let root = open_root_dir(image_handle, bs).unwrap();

    let memmap_file = root
        .open(
            "\\memmap.csv",
            uefi::types::EfiFileOpenMode::CreateReadWrite,
            uefi::types::EfiFileAttribute::None,
        )
        .unwrap();
    if save_memory_map(&memmap, memmap_file).is_err() {
        uefi_println!("Failed to save memory map");
    }

    match load_kernel(root, bs) {
        Ok(entry) => exit_and_jump(bs, image_handle, memmap.map_key, entry),
        Err(_) => {
            uefi_println!("Kernel load error");
            EfiStatus::EfiLoadError
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
