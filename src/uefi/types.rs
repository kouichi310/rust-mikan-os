use core::ffi::c_void;


pub type Char16  = u16;
pub type EfiPhysicalAddress = u64;
pub type EfiVirtualAddress = u64;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EfiHandle(pub *mut c_void);

#[repr(C)]
pub struct EfiTableHeader {
    signature: u64,
    revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}