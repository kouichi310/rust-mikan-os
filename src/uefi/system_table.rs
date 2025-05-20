use super::{boot_services::EfiBootServices, console::{EfiSimpleTextInputProtocol, EfiSimpleTextOutputProtocol}, types::*};

pub struct EfiRuntimeService {}
pub struct EfiConfigurationTable {}

#[repr(C)]
pub struct EfiSystemTable<'a> {
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

    pub boot_services: *mut EfiBootServices<'a>,

    pub number_of_table_entries: usize,
    pub config_table: *mut EfiConfigurationTable,

}

impl EfiSystemTable<'static> {
    pub fn con_out(&self) -> &mut EfiSimpleTextOutputProtocol {
        unsafe { &mut *self.con_out }
    }

    pub fn boot_services(&self) -> &EfiBootServices<'static> {
        unsafe { &*self.boot_services }
    }
}