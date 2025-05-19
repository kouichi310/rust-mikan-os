#[repr(C)]
pub struct SimpleTextOutputProtocol {
    _pad: [u8; 8],
    pub output_string: extern "efiapi" fn(*const Self, *const u16) -> usize,
}

pub fn print(con_out: &SimpleTextOutputProtocol, s: &[u16]) {
    (con_out.output_string)(con_out, s.as_ptr());
}