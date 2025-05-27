use core::{ffi::c_void, ptr::null_mut};

use crate::uefi_println;

use super::{
    guids::EfiGuid,
    memory::EfiMemoryDescriptor,
    status::EfiStatus,
    types::{
        EfiAllocateType, EfiHandle, EfiMemoryType, EfiPhysicalAddress, EfiTableHeader,
        NotImplemented,
    },
};

#[repr(C)]
pub struct EfiBootServices {
    pub hdr: EfiTableHeader,
    pub raise_tpl: NotImplemented,
    pub restore_tpl: NotImplemented,
    pub allocate_pages: extern "efiapi" fn(
        allocate_type: EfiAllocateType,
        memory_type: EfiMemoryType,
        pages: usize,
        memory: &EfiPhysicalAddress,
    ) -> EfiStatus,
    pub free_pages: NotImplemented,
    pub get_memory_map: extern "efiapi" fn(
        *mut usize,
        *mut EfiMemoryDescriptor,
        *mut usize,
        *mut usize,
        *mut u32,
    ) -> EfiStatus,
    allocate_pool:
        extern "efiapi" fn(pooltype: EfiMemoryType, size: usize, buffer: &mut *mut u8) -> EfiStatus,
    free_pool: extern "efiapi" fn(address: *const c_void) -> EfiStatus,
    create_event: NotImplemented,
    set_timer: NotImplemented,
    wait_for_event: NotImplemented,
    signal_event: NotImplemented,
    close_event: NotImplemented,
    check_event: NotImplemented,
    install_protocol_interface: NotImplemented,
    reinstall_protocol_interface: NotImplemented,
    uninstall_protocol_interface: NotImplemented,
    handle_protocol: NotImplemented,
    reserved: NotImplemented,
    register_protocol_notify: NotImplemented,
    locate_handle: NotImplemented,
    locate_device_path: NotImplemented,
    install_configuration_table: NotImplemented,
    load_image: NotImplemented,
    start_image: NotImplemented,
    exit: NotImplemented,
    unload_image: NotImplemented,
    exit_boot_service: extern "efiapi" fn(image_handle: EfiHandle, map_key: usize) -> EfiStatus,
    get_next_monotonic_count: NotImplemented,
    stall: NotImplemented,
    set_watchdog_timer: NotImplemented,
    connect_controller: NotImplemented,
    disconnect_controller: NotImplemented,
    open_protocol: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: *const EfiGuid,
        interface: &mut *mut c_void,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
        attributes: u32,
    ) -> EfiStatus,
    close_protocol: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: *const EfiGuid,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
    ) -> EfiStatus,
    open_protocol_infomation: NotImplemented,
    protocols_per_handle: NotImplemented,
    locate_handle_buffer: NotImplemented,
    locate_protocol: NotImplemented,
    install_multiple_protocol_interface: NotImplemented,
    uninstall_multiple_protocol_interface: NotImplemented,
    calculate_crc32: NotImplemented,
    copy_mem: NotImplemented,
    set_mem: NotImplemented,
    create_event_ex: NotImplemented,
}

impl EfiBootServices {
    pub fn get_memory_map(
        &self,
        buffer: &mut [u8],
    ) -> Result<(usize, usize, usize, u32), EfiStatus> {
        let mut map_size = buffer.len();
        let mut map_key = 0usize;
        let mut descriptor_size = 0usize;
        let mut descriptor_version = 0u32;

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

    pub fn open_protocol<T>(
        &self,
        handle: EfiHandle,
        protocol: &EfiGuid,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
        attributes: u32,
    ) -> Result<*mut T, EfiStatus> {
        let mut interface: *mut c_void = core::ptr::null_mut();
        let interface_ptr = &mut interface;

        let status = (self.open_protocol)(
            handle,
            protocol as *const EfiGuid,
            interface_ptr,
            agent_handle,
            controller_handle,
            attributes,
        );

        if status == EfiStatus::Success && !interface.is_null() {
            Ok(interface as *mut T)
        } else {
            uefi_println!("----debug info----");
            uefi_println!("Status: {:?}", status);
            uefi_println!("{:?}", handle);
            uefi_println!("{:?}", protocol);
            uefi_println!("{:?}", agent_handle);
            uefi_println!("{:?}", controller_handle);
            uefi_println!("{:?}", attributes);
            uefi_println!("----debug end----");
            Err(status)
        }
    }

    pub fn allocate_pages(
        &self,
        allocate_type: EfiAllocateType,
        memory_type: EfiMemoryType,
        mut pages: usize,
        mut memory: EfiPhysicalAddress,
    ) -> Result<EfiPhysicalAddress, EfiStatus> {
        if (pages % 0x1000) != 0 {
            //4KiBアライメント
            pages = pages.div_ceil(0x1000);
        }

        let _res = (self.allocate_pages)(allocate_type, memory_type, pages, &mut memory);
        if _res == EfiStatus::Success {
            Ok(memory)
        } else {
            Err(_res)
        }
    }

    pub fn allocate_pool(
        &self,
        pool_type: EfiMemoryType,
        size: usize,
    ) -> Result<*mut u8, EfiStatus> {
        let mut buffer: *mut u8 = null_mut();
        let _res = (self.allocate_pool)(pool_type, size, &mut buffer);

        if _res == EfiStatus::Success && !buffer.is_null() {
            Ok(buffer)
        } else {
            Err(_res)
        }
    }

    pub fn free_pool(&self, buffer: *const c_void) -> Result<EfiStatus, EfiStatus> {
        let _res = (self.free_pool)(buffer);

        if _res == EfiStatus::Success {
            Ok(_res)
        } else {
            Err(_res)
        }
    }

    pub fn exit_boot_service(
        &self,
        image_handle: EfiHandle,
        map_key: usize,
    ) -> Result<EfiStatus, EfiStatus> {
        let _res = (self.exit_boot_service)(image_handle, map_key);

        if _res == EfiStatus::Success {
            Ok(_res)
        } else {
            Err(_res)
        }
    }
}
