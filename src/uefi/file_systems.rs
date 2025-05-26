use crate::utils::fixed_buffer::encode_utf16_null_terminated;

use super::{
    guids::EfiGuid,
    status::EfiStatus,
    system_table::EfiSystemTable,
    types::{Char16, EfiFileAttribute, EfiFileOpenMode, EfiHandle, EfiMemoryType},
};
use core::{ffi::c_void, ptr};

pub const EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL: u32 = 0x00000001;

pub struct EfiFileIoToken {}

#[repr(C)]
pub struct EfiDevicePathProtocol {}

#[repr(C)]
pub struct EfiLoadedImageProtocol<'a> {
    revision: u32,
    parent_handle: EfiHandle,
    system_table: &'a EfiSystemTable,
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
        *const Char16,
        EfiFileOpenMode,
        EfiFileAttribute,
    ) -> EfiStatus,
    pub close: extern "efiapi" fn(&EfiFileProtocol) -> EfiStatus,
    pub delete: extern "efiapi" fn(&EfiFileProtocol) -> EfiStatus,
    pub read: extern "efiapi" fn(&EfiFileProtocol, &usize, &c_void) -> EfiStatus,
    pub write: extern "efiapi" fn(&EfiFileProtocol, &mut usize, *const c_void) -> EfiStatus,
    pub get_position: extern "efiapi" fn(&EfiFileProtocol, &u64) -> EfiStatus,
    pub set_position: extern "efiapi" fn(&EfiFileProtocol, &u64) -> EfiStatus,
    pub get_info: extern "efiapi" fn(&EfiFileProtocol, &EfiGuid, &usize, &c_void) -> EfiStatus,
    pub set_info: extern "efiapi" fn(&EfiFileProtocol, &EfiGuid, &usize, &c_void) -> EfiStatus,
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

impl EfiFileProtocol {
    pub fn open(
        &self,
        file_name: &str,
        mode: EfiFileOpenMode,
        attributes: EfiFileAttribute,
    ) -> Result<&EfiFileProtocol, EfiStatus> {
        let mut new_handle = ptr::null_mut();
        let new_handle_ptr = &mut new_handle;

        let mut utf16_buf = [0; 256];
        let file_name_ptr = encode_utf16_null_terminated(file_name, &mut utf16_buf)
            .expect("Failed to encode file name to UTF-16");

        let _res = (self.open)(self, new_handle_ptr, file_name_ptr, mode, attributes);

        if _res == EfiStatus::Success {
            unsafe { Ok(new_handle.as_ref().unwrap()) }
        } else {
            Err(_res)
        }
    }

    pub fn close(&self) -> Result<EfiStatus, EfiStatus> {
        let _res = (self.close)(self);
        if _res == EfiStatus::Success {
            Ok(_res)
        } else {
            Err(_res)
        }
    }

    pub fn read(&self, buffer_size: &usize, buffer: &c_void) -> EfiStatus {
        (self.read)(self, buffer_size, buffer)
    }

    pub fn write(&self, buffer_size: usize, buffer: *const u8) -> Result<usize, EfiStatus> {
        let mut written_size = buffer_size;

        let _res = (self.write)(self, &mut written_size, buffer as *const _);
        if _res == EfiStatus::Success {
            Ok(written_size)
        } else {
            Err(_res)
        }
    }
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
    pub fn open_volume(&mut self) -> Result<&EfiFileProtocol, EfiStatus> {
        let mut efi_file_proto = ptr::null_mut();
        let mut efi_file_proto_ptr = &mut efi_file_proto;

        let _res = (self.open_volume)(self, efi_file_proto_ptr);
        if _res == EfiStatus::Success {
            unsafe { Ok(efi_file_proto.as_ref().unwrap()) }
        } else {
            Err(_res)
        }
    }
}
