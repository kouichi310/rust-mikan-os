use core::fmt::{self, Write};

use crate::uefi::types::Char16;

pub struct FixedBuffer<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

#[warn(dead_code)]
impl<'a> FixedBuffer<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn clear(&mut self) {
        self.pos = 0;
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.pos]
    }

    pub fn len(&self) -> usize {
        self.pos
    }

    pub fn is_empty(&self) -> bool {
        self.pos == 0
    }
}

impl<'a> Write for FixedBuffer<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let remaining = self.buf.len().saturating_sub(self.pos);
        if bytes.len() > remaining {
            return Err(fmt::Error);
        }

        self.buf[self.pos..self.pos + bytes.len()].copy_from_slice(bytes);
        self.pos += bytes.len();
        Ok(())
    }
}

pub fn encode_utf16_null_terminated(
    utf8_str: &str,
    buffer: &mut [Char16],
) -> Option<*const Char16> {
    let mut i = 0;
    for code_unit in utf8_str.encode_utf16() {
        if i >= buffer.len() - 1 {
            return None; // 入らない
        }
        buffer[i] = code_unit;
        i += 1;
    }
    buffer[i] = 0; // null終端
    Some(buffer.as_ptr())
}
