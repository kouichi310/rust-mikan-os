use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
    ptr::NonNull,
};

use super::{boot_services::EfiBootServices, types::EfiMemoryType};

pub struct Allocator;

static mut EFI_BOOT_SERVICES: Option<NonNull<EfiBootServices>> = None;

pub fn init_allocator(boot_services: &EfiBootServices) {
    unsafe {
        EFI_BOOT_SERVICES = NonNull::new(boot_services as *const _ as *mut _);
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let memory_type = EfiMemoryType::EfiLoaderData;
        let size = layout.size();
        let alignment = layout.align();

        if alignment > 8 {
            panic!("Alignment greater than 8 is not supported");
        } else {
            let _res = unsafe {
                EFI_BOOT_SERVICES
                    .unwrap()
                    .as_ref()
                    .allocate_pool(memory_type, size)
            };

            _res.unwrap()
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if layout.align() > 8 {
            panic!("Alignment greater than 8 is not supported");
        } else {
            let _ = unsafe {
                EFI_BOOT_SERVICES
                    .unwrap()
                    .as_ref()
                    .free_pool(ptr as *const c_void)
            };
        }
    }
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

#[alloc_error_handler]
fn out_of_memory(layout: Layout) -> ! {
    panic!("Out of memory: {:?}", layout);
}
