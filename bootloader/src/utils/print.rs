use crate::uefi::types::Char16;
use alloc::vec::Vec;
use core::{
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};

use crate::uefi::console::EfiSimpleTextOutputProtocol;

static CON_OUT: AtomicPtr<EfiSimpleTextOutputProtocol> = AtomicPtr::new(null_mut());

pub fn setup_console(cout: &EfiSimpleTextOutputProtocol) {
    CON_OUT.store(cout as *const _ as *mut _, Ordering::SeqCst);
}

pub fn uefi_print_raw(s: &str) {
    let mut utf16: Vec<u16> = s.encode_utf16().collect();
    utf16.push(0); // NULL 終端
    unsafe {
        if let Some(con_out) = CON_OUT.load(Ordering::SeqCst).as_ref() {
            con_out.output_string(utf16.as_ptr());
        }
    }
}

pub fn encode_utf16_null_terminated(utf8_str: &str) -> Vec<Char16> {
    let mut buf: Vec<Char16> = utf8_str.encode_utf16().collect();
    buf.push(0);
    buf
}

#[macro_export]
macro_rules! uefi_println {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        use alloc::string::String;
        use core::fmt::Write;
        let mut s = String::new();
        let _ = write!(s, concat!($fmt, "\r\n") $(, $arg)*);
        $crate::utils::print::uefi_print_raw(&s);
    }};
}

#[macro_export]
macro_rules! uefi_print {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        use alloc::string::String;
        use core::fmt::Write;
        let mut s = String::new();
        $crate::utils::print::uefi_print_raw(&s);
    }};
}
