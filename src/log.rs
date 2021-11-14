pub const BIOS_SPEW: u32 = 0;
pub const BIOS_DEBUG: u32 = 1;
pub const BIOS_INFO: u32 = 2;
pub const BIOS_WARNING: u32 = 4;
pub const BIOS_ERR: u32 = 5;
pub const PRINTK_BUFFER_SIZE: usize = 255;

#[no_mangle]
pub unsafe extern "C" fn printk_u8(level: u32, msg: *const u8) {
    // CStr is only in std :(
    let mut len = 0;
    while *msg.add(len) != 0 && len < PRINTK_BUFFER_SIZE {
        len += 1;
    }
    let slice = core::slice::from_raw_parts(msg as *const u8, len);
    let str_slice = core::str::from_utf8_unchecked(slice);
    match level {
        BIOS_SPEW => log::trace!("{}", str_slice),
        BIOS_DEBUG => log::debug!("{}", str_slice),
        BIOS_INFO => log::info!("{}", str_slice),
        BIOS_WARNING => log::warn!("{}", str_slice),
        BIOS_ERR => log::error!("{}", str_slice),
        _ => {}
    }
}
