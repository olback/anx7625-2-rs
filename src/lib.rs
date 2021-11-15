#![no_std]

use {
    anx_fn_ptrs::AnxFnPtrs,
    core::{ffi::c_void, marker::PhantomData, mem::MaybeUninit},
    embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin},
    tinyrlibc as _,
};

mod anx_fn_ptrs;
pub mod c_compat;
mod i2c_read_write;

pub use {
    anx_fn_ptrs::{edid as Edid, edid_modes as EdidModes},
    i2c_read_write::I2CReadAndWrite,
};

pub struct Anx7625<
    Bus: I2CReadAndWrite<BusError>,
    VideoOnPin: OutputPin<Error = PinError>,
    VideoRstPin: OutputPin<Error = PinError>,
    OtgPin: OutputPin<Error = PinError>,
    BusError,
    PinError,
    Delay: DelayMs<u32>,
> {
    ptrs: AnxFnPtrs,
    video_on: VideoOnPin,
    video_rst: VideoRstPin,
    otg_on: OtgPin,
    _bus: PhantomData<Bus>,
    _bus_error: PhantomData<BusError>,
    _pin_error: PhantomData<PinError>,
    _d: PhantomData<Delay>,
}

impl<
        Bus: I2CReadAndWrite<BusError>,
        VideoOnPin: OutputPin<Error = PinError>,
        VideoRstPin: OutputPin<Error = PinError>,
        OtgPin: OutputPin<Error = PinError>,
        BusError,
        PinError,
        Delay: DelayMs<u32>,
    > Anx7625<Bus, VideoOnPin, VideoRstPin, OtgPin, BusError, PinError, Delay>
{
    pub fn new(video_on: VideoOnPin, video_rst: VideoRstPin, otg_on: OtgPin) -> Self {
        Self {
            ptrs: AnxFnPtrs::new(),
            video_on,
            video_rst,
            otg_on,
            _bus: PhantomData,
            _bus_error: PhantomData,
            _pin_error: PhantomData,
            _d: PhantomData,
        }
    }

    pub fn init(&mut self, bus: &mut Bus, delay: &mut Delay) -> Result<(), i32> {
        let mut bus = bus as &mut dyn I2CReadAndWrite<BusError>;
        let mut delay = delay as &mut dyn DelayMs<u32>;
        let mut video_on = &mut self.video_on as &mut dyn OutputPin<Error = PinError>;
        let mut video_rst = &mut self.video_rst as &mut dyn OutputPin<Error = PinError>;
        let mut otg_on = &mut self.otg_on as &mut dyn OutputPin<Error = PinError>;
        match self.ptrs.init(
            &mut bus as *mut _ as *mut c_void,
            &mut delay as *mut _ as *mut c_void,
            &mut video_on as *mut _ as *mut c_void,
            &mut video_rst as *mut _ as *mut c_void,
            &mut otg_on as *mut _ as *mut c_void,
        ) {
            0.. => Ok(()),
            err => Err(err),
        }
    }

    pub fn wait_hpd_event(&mut self, bus: &mut Bus, delay: &mut Delay) {
        let mut bus = bus as &mut dyn I2CReadAndWrite<BusError>;
        let mut delay = delay as &mut dyn DelayMs<u32>;
        self.ptrs.wait_hpd_event(
            &mut bus as *mut _ as *mut c_void,
            &mut delay as *mut _ as *mut c_void,
        )
    }

    pub fn dp_get_edid(
        &mut self,
        bus: &mut Bus,
        delay: &mut Delay,
    ) -> Result<Edid, (i32, MaybeUninit<Edid>)> {
        let mut bus = bus as &mut dyn I2CReadAndWrite<BusError>;
        let mut delay = delay as &mut dyn DelayMs<u32>;
        let mut edid_d = MaybeUninit::<anx_fn_ptrs::edid>::uninit();
        // Set struct to all 0
        unsafe { (&mut edid_d as *mut _ as *mut anx_fn_ptrs::edid).write_bytes(0, 1) };
        match self.ptrs.dp_get_edid(
            &mut bus as *mut _ as *mut c_void,
            &mut delay as *mut _ as *mut c_void,
            &mut edid_d as *mut _ as *mut anx_fn_ptrs::edid,
        ) {
            0.. => Ok(unsafe { edid_d.assume_init() }),
            err => Err((err, edid_d)),
        }
    }

    pub fn dp_start(
        &mut self,
        bus: &mut Bus,
        delay: &mut Delay,
        edid_d: &Edid,
        mode: EdidModes,
    ) -> Result<(), i32> {
        let mut bus = bus as &mut dyn I2CReadAndWrite<BusError>;
        let mut delay = delay as &mut dyn DelayMs<u32>;
        match self.ptrs.dp_start(
            &mut bus as *mut _ as *mut c_void,
            &mut delay as *mut _ as *mut c_void,
            edid_d as *const _,
            mode,
        ) {
            0.. => Ok(()),
            err => Err(err),
        }
    }
}
