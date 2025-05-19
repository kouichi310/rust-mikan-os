use super::types::*;
use super::status::EfiStatus;

#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    pub reset: extern "efiapi" fn(&Self, bool) -> EfiStatus,
    pub output_string: extern "efiapi" fn(&Self, *const Char16) -> EfiStatus,
    _reserved: [usize; 7],
}

pub struct EfiSimpleTextInputProtocol {}

impl EfiSimpleTextOutputProtocol {
    pub fn reset(&self, extended_verification: bool) -> EfiStatus {
        (self.reset)(self, extended_verification)
    }
    pub fn output_string(&self, s: *const Char16) -> EfiStatus {
        (self.output_string)(self, s)
    }
}