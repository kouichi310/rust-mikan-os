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

#[allow(dead_code)]
#[repr(C)]
pub enum EfiMemoryType {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
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

#[allow(dead_code)]
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
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiTime {
    year: u64,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    _pad1: u8,
    nanosecond: u32,
    time_zone: i16,
    daylight: u8,
    _pad2: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiFileInfo {
    pub size: u64,
    pub file_size: u64,
    pub physical_size: u64,
    pub create_time: EfiTime,
    pub last_access_time: EfiTime,
    pub modification_time: EfiTime,
    pub attribute: u64,
}

#[repr(C)]
pub enum EfiAllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

#[allow(dead_code)]
#[repr(C)]
pub enum EfiLocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
    ByHandle,
    ByDevicePath,
}

#[allow(dead_code)]
#[repr(C)]
pub enum EfiGraphicsPixelFormat {
    PixelRedGreenBlueReserved8BitPerClolor,
    PixelBlueGreenRedReserved8BitPerColor,
    PixelBitMask,
    PixelBitOnly,
    PixelFormatMMax,
}

#[allow(dead_code)]
#[repr(C)]
pub enum EfiGraphicsOutputBltOperation {
    BltVideoFill,
    BltVideoToVltBuffer,
    BltBufferToVideo,
    BitVideoToVideo,
    GraphicsOutputBltOperationMax,
}

#[allow(dead_code)]
#[repr(C)]
pub struct EfiGraphicsOutputBltPixel {
    blue: u8,
    green: u8,
    red: u8,
    _reserved: u8,
}

#[allow(dead_code)]
#[repr(C)]
pub struct EfiPixelBitmask {
    red_mask: u32,
    green_mask: u32,
    blue_mask: u32,
    reserved_mask: u32,
}

#[repr(C)]
pub struct EfiGraphicsOutputProtocolMode<'a> {
    pub max_mode: u32,
    pub mode: u32,
    pub info: &'a EfiGraphicsOutputModeInformation,
    pub size_of_info: usize,
    pub frame_buffer_base: EfiPhysicalAddress,
    pub frame_buffer_size: usize,
}

#[allow(dead_code)]
#[repr(C)]
pub struct EfiGraphicsOutputModeInformation {
    version: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pixel_format: EfiGraphicsPixelFormat,
    pixel_information: EfiPixelBitmask,
    pixel_per_scan_line: u32,
}
