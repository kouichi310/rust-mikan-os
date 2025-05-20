use core::ffi::c_void;
use super::{guids::EfiGuid, status::EfiStatus, system_table::EfiSystemTable, types::{Char16, EfiHandle, EfiMemoryType}};

pub const EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL: u32 = 0x00000001;

pub struct EfiFileIoToken {}

#[repr(C)]
pub struct EfiDevicePathProtocol {}

#[repr(C)]
pub struct EfiLoadedImageProtocol<'a> {
    revision: u32,
    parent_handle: EfiHandle,
    system_table: EfiSystemTable<'a>,
    pub device_handle: EfiHandle,
    file_path: &'a EfiDevicePathProtocol,
    reserved: &'a c_void,
    load_options_size: u32,
    load_options: &'a c_void,
    image_base: &'a c_void,
    image_size: u64,
    image_code_type: EfiMemoryType,
    image_data_type: EfiMemoryType,
    unload: extern "efiapi" fn(image_handle: EfiHandle) -> EfiStatus,
}


#[repr(C)]
pub struct EfiFileProtocol {
    pub revision: u64,
    pub open: extern "efiapi" fn(
        &EfiFileProtocol, 
        &mut *mut EfiFileProtocol,
        &Char16,
        u64,
        u64,
    ) -> EfiStatus,
    pub close: extern "efiapi" fn(&EfiFileProtocol) -> EfiStatus,
    pub delete: extern "efiapi" fn(&EfiFileProtocol) -> EfiStatus,
    pub read: extern "efiapi" fn(
        &EfiFileProtocol, 
        &usize,
        &c_void,
    ) -> EfiStatus,
    pub write: extern "efiapi" fn(
        &EfiFileProtocol, 
        &usize,
        &c_void,
    ) -> EfiStatus,
    pub get_position: extern "efiapi" fn(&EfiFileProtocol, &u64) -> EfiStatus,
    pub set_position: extern "efiapi" fn(&EfiFileProtocol, &u64) -> EfiStatus,
    pub get_info: extern "efiapi" fn(
        &EfiFileProtocol,
        &EfiGuid,
        &usize,
        &c_void,
    ) -> EfiStatus,
    pub set_info: extern "efiapi" fn(
        &EfiFileProtocol,
        &EfiGuid,
        &usize,
        &c_void,
    ) -> EfiStatus,
    pub flash: extern "efiapi" fn(&EfiFileProtocol) -> EfiStatus,
    pub open_ex: extern "efiapi" fn(
        &EfiFileProtocol,
        &&EfiFileProtocol,
        &Char16,
        u64,
        u64,
        &EfiFileIoToken,
    ) -> EfiStatus,
    pub read_ex: extern "efiapi" fn(&EfiFileProtocol, &EfiFileIoToken) -> EfiStatus,
    pub write_ex: extern "efiapi" fn(&EfiFileProtocol, &EfiFileIoToken) -> EfiStatus,
    pub flash_ex: extern "efiapi" fn(&EfiFileProtocol, &EfiFileIoToken) -> EfiStatus,
}

#[repr(C)]
pub struct EfiSimpleFileSystemProtocol {
    revision: u64,
    open_volume: extern "efiapi" fn(
        this: *mut EfiSimpleFileSystemProtocol,
        root: &mut *mut EfiFileProtocol,
    ) -> EfiStatus,
}

impl EfiSimpleFileSystemProtocol {
    pub fn open_volume(&mut self, root: &mut *mut EfiFileProtocol) -> EfiStatus {
        (self.open_volume)(self, root)
    }
}