#[repr(C)]
pub struct EfiGuid {
    data_1: u32,
    data_2: u16,
    data_3: u16,
    data_4: [u8;8]
}

pub const EFI_LOADED_IMAGE_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data_1: 0x5b1b31a1,
    data_2: 0x9652,
    data_3: 0x11d2,
    data_4: [0x8e, 0x3f, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]
};

pub const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data_1: 0x964e5b22,
    data_2: 0x9652,
    data_3: 0x11d2,
    data_4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]
};