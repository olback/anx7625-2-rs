#![allow(unused)]

// Make a new "public" generic struct so that Anx can be generated

pub struct Bus {
    pub _b: u8,
}
impl Bus {
    pub fn new() -> Self {
        Self { _b: 0 }
    }

    fn write(&mut self, addr: u8, data: &[u8]) -> Result<usize, ()> {
        println!("I2C: Writing to 0x{:0x} {:?}", addr, data);
        Ok(10)
    }

    fn read(&mut self, addr: u8, buf: &mut [u8]) -> Result<usize, ()> {
        Ok(10)
    }
}
pub struct Delay {
    pub _d: u8,
}
impl Delay {
    pub fn new() -> Self {
        Self { _d: 0 }
    }
}

trait DelayMs {
    fn delay_ms(&mut self, ms: u32);
}

impl DelayMs for Delay {
    fn delay_ms(&mut self, ms: u32) {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64))
    }
}

use core::ffi::c_void;

include!("../gen/anx7625.rs");

macro_rules! cast_ptr {
    ($expr:expr, $ty:ty) => {
        $expr as *const _ as *const $ty
    };
}

macro_rules! cast_ptr_mut {
    ($expr:expr, $ty:ty) => {
        $expr as *mut _ as *mut $ty
    };
}

#[repr(C)]
pub struct Anx {
    delay: unsafe extern "C" fn(anx: *mut Self, delay: *mut c_void, ms: u32) -> (),
    set_video_on: unsafe extern "C" fn(anx: *mut Self, state: bool) -> u8,
    set_video_rst: unsafe extern "C" fn(anx: *mut Self, state: bool) -> u8,
    set_otg: unsafe extern "C" fn(anx: *mut Self, state: bool) -> u8,
    i2c_writeb: unsafe extern "C" fn(
        anx: *mut Self,
        bus: *mut c_void,
        addr: u8,
        data: *const u8,
        len: usize,
    ) -> u8,
    i2c_readb: unsafe extern "C" fn(
        anx: *mut Self,
        bus: *mut c_void,
        addr: u8,
        data: *mut u8,
        len: *mut usize,
    ) -> u8,
}

unsafe extern "C" fn delay(anx: *mut Anx, delay: *mut c_void, ms: u32) {
    // let mut d: &mut &mut dyn DelayMs = core::mem::transmute(delay);
    // (**d).delay_ms(ms);
    // Ah, yes
    (**core::mem::transmute::<_, &mut &mut dyn DelayMs>(delay)).delay_ms(ms);
}

unsafe extern "C" fn set_video_on(anx: *mut Anx, state: bool) -> u8 {
    0
}

unsafe extern "C" fn set_video_rst(anx: *mut Anx, state: bool) -> u8 {
    0
}

unsafe extern "C" fn set_otg(anx: *mut Anx, state: bool) -> u8 {
    0
}

unsafe extern "C" fn i2c_writeb(
    anx: *mut Anx,
    bus: *mut c_void,
    addr: u8,
    data: *const u8,
    len: usize,
) -> u8 {
    let slice = core::slice::from_raw_parts(data, len);
    match (&mut *cast_ptr_mut!(bus, Bus)).write(addr, slice) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

unsafe extern "C" fn i2c_readb(
    anx: *mut Anx,
    bus: *mut c_void,
    addr: u8,
    data: *mut u8,
    len: *mut usize,
) -> u8 {
    let slice = core::slice::from_raw_parts_mut(data, *len);
    match (&mut *cast_ptr_mut!(bus, Bus)).read(addr, slice) {
        Ok(bytes_read) => {
            *len = bytes_read;
            1
        }
        Err(_) => 0,
    }
}

impl Anx {
    pub fn new() -> Self {
        Self {
            delay,
            set_video_on,
            set_video_rst,
            set_otg,
            i2c_writeb,
            i2c_readb,
        }
    }

    pub fn init(&mut self, bus: &mut Bus, delay: &mut Delay) -> Result<u8, ()> {
        let res = unsafe {
            anx_init(
                self,
                cast_ptr_mut!(bus, c_void),
                cast_ptr_mut!(&mut (delay as &mut dyn DelayMs), c_void),
            )
        };
        Ok(res)
    }
}
