use super::{
    status::EfiStatus,
    types::{
        EfiGraphicsOutputBltOperation, EfiGraphicsOutputBltPixel, EfiGraphicsOutputModeInformation,
        EfiGraphicsOutputProtocolMode,
    },
};

#[allow(dead_code)]
#[repr(C)]
pub struct EfiGraphicsOutputProtocol<'a> {
    pub query_mode: extern "efiapi" fn(
        this: &Self,
        mode_number: u32,
        size_of_info: &usize,
        info: &&EfiGraphicsOutputModeInformation,
    ) -> EfiStatus,
    pub set_mode: extern "efiapi" fn(this: &Self, mode_number: u32) -> EfiStatus,
    pub blt: extern "efiapi" fn(
        this: &Self,
        blt_buffer: EfiGraphicsOutputBltPixel,
        bit_operation: EfiGraphicsOutputBltOperation,
        source_x: usize,
        source_y: usize,
        destination_x: usize,
        destination_y: usize,
        width: usize,
        height: usize,
        delta: usize,
    ) -> EfiStatus,
    pub mode: &'a EfiGraphicsOutputProtocolMode<'a>,
}
