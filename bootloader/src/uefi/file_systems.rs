use crate::utils::print::encode_utf16_null_terminated;

use super::{
    guids::{EFI_FILE_INFO_GUID, EfiGuid},
    status::EfiStatus,
    system_table::EfiSystemTable,
    types::{Char16, EfiFileAttribute, EfiFileInfo, EfiFileOpenMode, EfiHandle, EfiMemoryType},
};
use core::{ffi::c_void, mem::size_of, ptr};

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
    pub close: extern "efiapi" fn(this: &EfiFileProtocol) -> EfiStatus,
    pub delete: extern "efiapi" fn(&EfiFileProtocol) -> EfiStatus,
    pub read: extern "efiapi" fn(&EfiFileProtocol, &usize, *mut c_void) -> EfiStatus,
    pub write: extern "efiapi" fn(&EfiFileProtocol, &mut usize, *const c_void) -> EfiStatus,
    pub get_position: extern "efiapi" fn(&EfiFileProtocol, &u64) -> EfiStatus,
    pub set_position: extern "efiapi" fn(&EfiFileProtocol, &u64) -> EfiStatus,
    pub get_info: extern "efiapi" fn(&EfiFileProtocol, &EfiGuid, &usize, *mut c_void) -> EfiStatus,
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

        let file_name_utf16 = encode_utf16_null_terminated(file_name);
        let file_name_ptr = file_name_utf16.as_ptr();

        let _res = (self.open)(self, new_handle_ptr, file_name_ptr, mode, attributes);

        if _res == EfiStatus::Success {
            unsafe {
                if let Some(handle_ref) = new_handle.as_ref() {
                    Ok(handle_ref)
                } else {
                    Err(EfiStatus::EfiDeviceError) // Return an appropriate error if null
                }
            }
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

    pub fn read(&self, buffer_size: usize, load_address: u64) -> Result<EfiStatus, EfiStatus> {
        let _kernel_loaded_address = load_address as *mut u64;
        let _res = (self.read)(self, &buffer_size, _kernel_loaded_address as *mut _);

        if _res == EfiStatus::Success {
            Ok(_res)
        } else {
            Err(_res)
        }
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

    pub fn get_info(&self) -> Result<EfiFileInfo, EfiStatus> {
        let mut buffer = [0u8; size_of::<EfiFileInfo>() + size_of::<Char16>() * 23];

        let buffer_ptr = buffer.as_mut_ptr() as *mut c_void;
        let _res = (self.get_info)(self, &EFI_FILE_INFO_GUID, &(buffer.len()), buffer_ptr);
        if _res == EfiStatus::Success {
            let file_info = unsafe { (buffer.as_ptr() as *const EfiFileInfo).as_ref().unwrap() };
            Ok(*file_info)
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
        let efi_file_proto_ptr = &mut efi_file_proto;

        let _res = (self.open_volume)(self, efi_file_proto_ptr);
        if _res == EfiStatus::Success {
            unsafe { Ok(efi_file_proto.as_ref().unwrap()) }
        } else {
            Err(_res)
        }
    }
}
