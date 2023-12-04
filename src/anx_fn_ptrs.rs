#![allow(unused)]

use {
    super::I2CReadAndWrite,
    core::{convert::Infallible, ffi::c_void, mem::MaybeUninit},
    embedded_hal::{
        blocking::{
            delay::DelayMs,
            i2c::{Read as I2CRead, Write as I2CWrite},
        },
        digital::v2::{OutputPin, PinState},
    },
};

// #[path = "../gen/anx7625.rs"]
// #[allow(non_camel_case_types)]
// /// cbindgen:ignore
// mod gen;
// use gen::*;

include!("../gen/anx7625.rs");

#[repr(C)]
pub struct AnxFnPtrs {
    delay: unsafe extern "C" fn(anx: *mut Self, delay: *mut c_void, ms: u32) -> (),
    set_pin: unsafe extern "C" fn(anx: *mut Self, pin: *mut c_void, state: bool) -> i32,
    i2c_writeb: unsafe extern "C" fn(
        anx: *mut AnxFnPtrs,
        bus: *mut c_void,
        addr: u8,
        offset: u8,
        val: u8,
    ) -> i32,
    i2c_readb: unsafe extern "C" fn(
        anx: *mut AnxFnPtrs,
        bus: *mut c_void,
        addr: u8,
        offset: u8,
        data: *mut u8,
    ) -> i32,
    i2c_read_bytes: unsafe extern "C" fn(
        anx: *mut AnxFnPtrs,
        bus: *mut c_void,
        addr: u8,
        offset: u8,
        data: *mut u8,
        len: usize,
    ) -> i32,
}

unsafe extern "C" fn delay(anx: *mut AnxFnPtrs, delay: *mut c_void, ms: u32) {
    // Ah, yes
    // log::trace!("delay delay:{:p} time:{}ms", delay, ms);
    let mut d: &mut &mut dyn DelayMs<u32> = core::mem::transmute(delay);
    d.delay_ms(ms);
    // (**core::mem::transmute::<_, &mut &mut dyn DelayMs<u32>>(delay)).delay_ms(ms);
}

unsafe extern "C" fn set_pin(anx: *mut AnxFnPtrs, pin: *mut c_void, state: bool) -> i32 {
    // log::trace!("set_pin pin:{:p} state:{}", pin, state);
    let mut p: &mut &mut dyn OutputPin<Error = Infallible> = core::mem::transmute(pin);
    p.set_state(PinState::from(state));
    // (**core::mem::transmute::<_, &mut &mut dyn OutputPin<Error = Infallible>>(pin))
    //     .set_state(PinState::from(state));
    0
}

unsafe extern "C" fn i2c_writeb(
    anx: *mut AnxFnPtrs,
    bus: *mut c_void,
    addr: u8,
    offset: u8,
    val: u8,
) -> i32 {
    // log::trace!("i2c_writeb addr:{} offset:{} val:{}", addr, offset, val);
    let mut b: &mut &mut dyn I2CReadAndWrite<Infallible> = core::mem::transmute(bus);
    match b.write(addr, &[offset, val]) {
        Ok(_) => 0,
        Err(_) => {
            log::error!("i2c_writeb write");
            -1
        }
    }
}

unsafe extern "C" fn i2c_readb(
    anx: *mut AnxFnPtrs,
    bus: *mut c_void,
    addr: u8,
    offset: u8,
    data: *mut u8,
) -> i32 {
    // log::trace!("i2c_readb addr:{} offset:{}", addr, offset);
    let mut b: &mut &mut dyn I2CReadAndWrite<Infallible> = core::mem::transmute(bus);
    let mut slice = [0u8];
    if let Err(e) = b.write(addr, &[offset]) {
        log::error!("i2c_readb write");
        return -1;
    }
    match b.read(addr, &mut slice) {
        Ok(_) => {
            data.write(slice[0]); // *data  = slice[0];
            0
        }
        Err(_) => {
            log::error!("i2c_readb read");
            -1
        }
    }
}

unsafe extern "C" fn i2c_read_bytes(
    anx: *mut AnxFnPtrs,
    bus: *mut c_void,
    addr: u8,
    offset: u8,
    data: *mut u8,
    len: usize,
) -> i32 {
    // log::trace!("i2c_read_bytes addr:{} offset:{}", addr, offset);
    let mut b: &mut &mut dyn I2CReadAndWrite<Infallible> = core::mem::transmute(bus);
    if let Err(e) = b.write(addr, &[offset]) {
        log::error!("i2c_read_bytes offset write");
        return -1;
    }
    let mut slice = core::slice::from_raw_parts_mut(data, len);
    match b.read(addr, &mut slice) {
        Ok(_) => 0,
        Err(_) => {
            log::error!("i2c_read_bytes read");
            -1
        }
    }
}

impl AnxFnPtrs {
    pub fn new() -> Self {
        Self {
            delay,
            set_pin,
            i2c_writeb,
            i2c_readb,
            i2c_read_bytes,
        }
    }

    pub(crate) fn init(
        &mut self,
        bus: *mut c_void,
        delay: *mut c_void,
        video_on: *mut ::core::ffi::c_void,
        video_rst: *mut ::core::ffi::c_void,
        otg_on: *mut ::core::ffi::c_void,
    ) -> i32 {
        unsafe { anx7625_init(self as *mut _, bus, delay, video_on, video_rst, otg_on) }
    }

    pub(crate) fn wait_hpd_event(&mut self, bus: *mut c_void, delay: *mut c_void) {
        unsafe { anx7625_wait_hpd_event(self as *mut _, bus, delay) }
    }

    pub(crate) fn dp_get_edid(
        &mut self,
        bus: *mut c_void,
        delay: *mut c_void,
        edid_d: *mut edid,
    ) -> i32 {
        unsafe { anx7625_dp_get_edid(self as *mut _, bus, delay, edid_d) }
    }
    pub(crate) fn dp_start(
        &mut self,
        bus: *mut c_void,
        delay: *mut c_void,
        edid_d: *const edid,
        mode: edid_modes,
    ) -> i32 {
        unsafe { anx7625_dp_start(self as *mut _, bus, delay, edid_d, mode) }
    }
}
