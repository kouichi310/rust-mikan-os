use super::{memory::EfiMemoryDescriptor, status::EfiStatus};


#[repr(C)]
pub struct EfiBootServices {
    pub hdr: super::types::EfiTableHeader,
    pub raise_tpl: usize,
    pub restore_tpl: usize,
    pub allocate_pages: usize,
    pub free_pages: usize,
    pub get_memory_map: extern "efiapi" fn(
        *mut usize,
        *mut EfiMemoryDescriptor,
        *mut usize,
        *mut usize,
        *mut u32,
    ) -> EfiStatus,
    pub allocate_pool: usize,
    pub free_pool: usize,
}

impl EfiBootServices {
    pub fn get_memory_map(
        &self,
        buffer: &mut [u8],
    ) -> Result<(usize, usize, usize, u32), EfiStatus> {
        let mut map_size = buffer.len();
        let mut map_key = 0usize;
        let mut descriptor_size = 0usize;
        let mut descriptor_version= 0u32;

        let status = (self.get_memory_map)(
                &mut map_size,
                buffer.as_mut_ptr() as *mut EfiMemoryDescriptor,
                &mut map_key,
                &mut descriptor_size,
                &mut descriptor_version,
            );

        if status != EfiStatus::Success {
            return Err(status);
        }
        Ok((map_size, map_key, descriptor_size, descriptor_version))
    }
}