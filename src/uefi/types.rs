#[warn(dead_code)]
use core::ffi::c_void;

pub type Char16 = u16;
pub type EfiPhysicalAddress = u64;
pub type EfiVirtualAddress = u64;
pub type NotImplemented = usize;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct EfiHandle(pub *mut c_void);

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
pub enum EfiMemoryType {
    EfiReservedMemoryType = 0,
    EfiLoaderCode = 1,
    EfiLoaderData = 2,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiUnacceptedMemoryType,
    EfiMaxMemoryType,
}

#[derive(Clone, Copy, Debug)]
#[repr(u64)]
pub enum EfiFileOpenMode {
    Read = 0x1,
    ReadWrite = 0x2 | 0x1,
    CreateReadWrite = 0x8000_0000_0000_0000 | 0x2 | 0x1,
}

#[derive(Clone, Copy, Debug)]
#[repr(u64)]
pub enum EfiFileAttribute {
    None = 0x0,
    ReadOnly = 0x1,
    Hidden = 0x2,
    System = 0x4,
    Reserved = 0x8,
    Directory = 0x10,
    Archive = 0x20,
    ValidAttributes = 0x37,
}