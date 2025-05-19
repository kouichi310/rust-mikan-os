use core::ffi::c_void;

use super::{guids::EfiGuid, memory::EfiMemoryDescriptor, status::EfiStatus, types::{EfiHandle, NotImplemented}};


#[repr(C)]
pub struct EfiBootServices<'a> {
    pub hdr: super::types::EfiTableHeader,
    pub raise_tpl: NotImplemented,
    pub restore_tpl: NotImplemented,
    pub allocate_pages: NotImplemented,
    pub free_pages: NotImplemented,
    pub get_memory_map: extern "efiapi" fn(
        *mut usize,
        *mut EfiMemoryDescriptor,
        *mut usize,
        *mut usize,
        *mut u32,
    ) -> EfiStatus,
    pub allocate_pool: NotImplemented,
    pub free_pool: NotImplemented,
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
    reserved: &'a c_void,
    register_protocol_notify: NotImplemented,
    locate_handle: NotImplemented,
    locate_device_path: NotImplemented,
    install_configuration_table: NotImplemented,
    load_image: NotImplemented,
    start_image: NotImplemented,
    exit: NotImplemented,
    unload_image: NotImplemented,
    exit_boot_service: NotImplemented,
    get_next_monotonic_count: NotImplemented,
    stall: NotImplemented,
    set_watchdog_timer: NotImplemented,
    connect_controller: NotImplemented,
    disconnect_controller: NotImplemented,
    open_protocol: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: *const EfiGuid,
        interface: *mut *mut c_void,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
        attributes: u32,
    ) -> EfiStatus,
    close_protocol: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: *const EfiGuid,
        agent_handle: EfiHandle,
        cotroller_handle: EfiHandle,
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

impl EfiBootServices<'static> {
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

    pub fn open_protocol(
        &self,
        handle: EfiHandle,
        protocol: &EfiGuid,
        interface: *mut *mut c_void,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
        attributes: u32,
    ) -> EfiStatus {
        (self.open_protocol)(
            handle,
            protocol as *const EfiGuid,
            interface,
            agent_handle,
            controller_handle,
            attributes,
        )
    }
}