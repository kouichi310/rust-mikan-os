use crate::uefi::console::EfiSimpleTextOutputProtocol;
use spin::Once;

static CON_OUT: Once<&'static EfiSimpleTextOutputProtocol> = Once::new();

pub fn set_con_out(ptr: &'static EfiSimpleTextOutputProtocol) {
    CON_OUT.call_once(|| ptr);
}

fn str_to_utf16_buf(s: &str, buf: &mut [u16]) -> usize {
    let mut len = 0;
    for ch in s.encode_utf16() {
        if len >= buf.len() - 1 {
            break;
        }
        buf[len] = ch;
        len += 1;
    }
    buf[len] = 0;
    len + 1
}

pub fn uefi_print_raw(s: &str) {
    if let Some(con_out) = CON_OUT.get() {
        let mut buf = [0u16; 256];
        str_to_utf16_buf(s, &mut buf);
        con_out.output_string(buf.as_ptr());
    }
}

#[macro_export]
macro_rules! uefi_println {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        use core::fmt::Write;
        let mut buf = [0u8; 256];
        let mut fb = $crate::utils::fixed_buffer::FixedBuffer::new(&mut buf);
        let _ = write!(fb, concat!($fmt, "\r\n") $(, $arg)*);
        $crate::utils::print::uefi_print_raw(core::str::from_utf8(fb.as_bytes()).unwrap_or("[utf8 error]"));
    }};
}
