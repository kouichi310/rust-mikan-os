use super::{boot_services::EfiBootServices, console::{EfiSimpleTextInputProtocol, EfiSimpleTextOutputProtocol}, types::*};

pub struct EfiRuntimeService {}
pub struct EfiConfigurationTable {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EfiTableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct SystemTable {
    pub hdr: EfiTableHeader,
    pub firmware_vendor: *const Char16,
    pub firmware_revision: u32,
    
    pub console_in_handle: EfiHandle,
    pub con_in: *mut EfiSimpleTextInputProtocol,
    pub console_out_handle: EfiHandle,
    pub con_out: *mut EfiSimpleTextOutputProtocol,

    pub std_err_handle: EfiHandle,
    pub std_err: *mut EfiSimpleTextOutputProtocol,
    pub runtime_services: *mut EfiRuntimeService,

    pub boot_services: *mut EfiBootServices,

    pub number_of_table_entries: usize,
    pub config_table: *mut EfiConfigurationTable,

}

impl SystemTable {
    pub fn con_out(&self) -> &mut EfiSimpleTextOutputProtocol {
        unsafe { &mut *self.con_out }
    }

    pub fn boot_services(&self) -> &EfiBootServices {
        unsafe { &*self.boot_services }
    }
}