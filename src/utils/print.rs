use crate::uefi::console::EfiSimpleTextOutputProtocol;

pub static mut CON_OUT: Option<&EfiSimpleTextOutputProtocol> = None;

pub fn set_con_out(ptr: &'static EfiSimpleTextOutputProtocol) {
    unsafe {
        CON_OUT = Some(ptr);
    }
}

pub fn uefi_print_raw(con_out: &EfiSimpleTextOutputProtocol, s: &str) {
    let mut buf: [u16; 256] = [0; 256];
    let mut len = 0;
    for ch in s.encode_utf16() {
        if len < buf.len() {
            buf[len] = ch;
            len += 1;
        } else {
            break;
        }
    }
    buf[len] = 0;
    con_out.output_string(buf.as_ptr());
}

#[macro_export]
macro_rules! println {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        use core::fmt::Write;
        let mut buf = [0u8; 256];
        let mut fb = $crate::utils::fixed_buffer::FixedBuffer::new(&mut buf);
        let _ = write!(fb, concat!($fmt, "\r\n") $(, $arg)*);
        unsafe {
            if let Some(con_out) = $crate::utils::print::CON_OUT {
                $crate::utils::print::uefi_print_raw(con_out, core::str::from_utf8(fb.as_bytes()).unwrap_or("[utf8 error]"));
            }
        }
    }};
}
