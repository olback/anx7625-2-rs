pub const BIOS_SPEW: u32 = 0;
pub const BIOS_DEBUG: u32 = 1;
pub const BIOS_INFO: u32 = 2;
pub const BIOS_WARNING: u32 = 4;
pub const BIOS_ERR: u32 = 5;
pub const PRINTK_BUFFER_SIZE: usize = 255;

unsafe fn str_len(msg: *const u8) -> usize {
    let mut len = 0;
    while *msg.add(len) != 0 && len < PRINTK_BUFFER_SIZE {
        len += 1;
    }
    len
}

#[no_mangle]
pub unsafe extern "C" fn strcopy_compat(dst: *mut u8, src: *const u8) -> *mut u8 {
    let len = str_len(src);
    core::ptr::copy_nonoverlapping(src, dst, len);
    dst
}

#[no_mangle]
pub unsafe extern "C" fn isupper_compat(c: u8) -> i32 {
    match core::char::from_u32_unchecked(c as u32).is_ascii_uppercase() {
        true => 1,
        false => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn assert_panic(b: bool, msg: *const u8) {
    if b == false {
        let len = str_len(msg);
        let slice = core::slice::from_raw_parts(msg as *const u8, len);
        let str_slice = core::str::from_utf8_unchecked(slice);
        panic!("{}", str_slice)
    }
}

#[no_mangle]
pub unsafe extern "C" fn printk_u8(level: u32, msg: *const u8) {
    // CStr is only in std :(
    let len = str_len(msg);
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
