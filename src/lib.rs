#![no_std]

use core::mem::MaybeUninit;

use {
    anx_fn_ptrs::AnxFnPtrs,
    core::{ffi::c_void, marker::PhantomData},
    embedded_hal::{
        blocking::{
            delay::DelayMs,
            i2c::{Read as I2CRead, SevenBitAddress, Write as I2CWrite},
        },
        digital::v2::OutputPin,
    },
    tinyrlibc as _,
};

mod anx_fn_ptrs;
pub mod log;

pub trait I2CReadAndWrite<E>:
    I2CRead<SevenBitAddress, Error = E> + I2CWrite<SevenBitAddress, Error = E>
{
}

impl<T, E> I2CReadAndWrite<E> for T
where
    T: I2CRead<SevenBitAddress, Error = E>,
    T: I2CWrite<SevenBitAddress, Error = E>,
{
}

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
    ) -> Result<anx_fn_ptrs::edid, i32> {
        let mut bus = bus as &mut dyn I2CReadAndWrite<BusError>;
        let mut delay = delay as &mut dyn DelayMs<u32>;
        let mut edid_d = MaybeUninit::<anx_fn_ptrs::edid>::uninit();
        match self.ptrs.dp_get_edid(
            &mut bus as *mut _ as *mut c_void,
            &mut delay as *mut _ as *mut c_void,
            &mut edid_d as *mut _ as *mut anx_fn_ptrs::edid,
        ) {
            0.. => Ok(unsafe { edid_d.assume_init() }),
            err => Err(err),
        }
    }
}
