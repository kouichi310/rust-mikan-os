use super::output::SimpleTextOutputProtocol;

#[repr(C)]
pub struct SystemTable {
    _pad1: [u8; 52],
    pub con_out: *const SimpleTextOutputProtocol,
}

impl SystemTable {
    pub unsafe fn con_out(&self) -> &SimpleTextOutputProtocol {
        unsafe { &*self.con_out }
    }
}