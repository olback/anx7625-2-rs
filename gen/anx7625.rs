/* automatically generated by rust-bindgen 0.59.1 */

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum edid_modes {
    EDID_MODE_640x480_60Hz = 0,
    EDID_MODE_720x480_60Hz = 1,
    EDID_MODE_800x600_59Hz = 2,
    EDID_MODE_1024x768_60Hz = 3,
    EDID_MODE_1280x768_60Hz = 4,
    EDID_MODE_1280x720_60Hz = 5,
    EDID_MODE_1920x1080_60Hz = 6,
    NUM_KNOWN_MODES = 7,
    EDID_MODE_AUTO = 8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct edid_mode {
    pub name: *const i8,
    pub pixel_clock: u32,
    pub lvds_dual_channel: i32,
    pub refresh: u32,
    pub ha: u32,
    pub hbl: u32,
    pub hso: u32,
    pub hspw: u32,
    pub hborder: u32,
    pub va: u32,
    pub vbl: u32,
    pub vso: u32,
    pub vspw: u32,
    pub vborder: u32,
    pub voffset: u32,
    pub phsync: u8,
    pub pvsync: u8,
    pub x_mm: u32,
    pub y_mm: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct edid {
    pub framebuffer_bits_per_pixel: u32,
    pub panel_bits_per_color: u32,
    pub panel_bits_per_pixel: u32,
    pub mode: edid_mode,
    pub mode_is_supported: [u8; 7usize],
    pub link_clock: u32,
    pub x_resolution: u32,
    pub y_resolution: u32,
    pub bytes_per_line: u32,
    pub hdmi_monitor_detected: i32,
    pub ascii_string: [i8; 14usize],
    pub manufacturer_name: [i8; 4usize],
}
extern "C" {
    pub fn anx7625_init(
        anx: *mut AnxFnPtrs,
        bus: *mut ::core::ffi::c_void,
        delay: *mut ::core::ffi::c_void,
        video_on: *mut ::core::ffi::c_void,
        video_rst: *mut ::core::ffi::c_void,
        otg_on: *mut ::core::ffi::c_void,
    ) -> i32;
}
extern "C" {
    pub fn anx7625_wait_hpd_event(
        anx: *mut AnxFnPtrs,
        bus: *mut ::core::ffi::c_void,
        delay: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn anx7625_dp_get_edid(
        anx: *mut AnxFnPtrs,
        bus: *mut ::core::ffi::c_void,
        delay: *mut ::core::ffi::c_void,
        out: *mut edid,
    ) -> i32;
}
extern "C" {
    pub fn anx7625_dp_start(
        anx: *mut AnxFnPtrs,
        bus: *mut ::core::ffi::c_void,
        delay: *mut ::core::ffi::c_void,
        edid: *const edid,
        mode: edid_modes,
    ) -> i32;
}
