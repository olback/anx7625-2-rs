use {
    ::log::{Level, Metadata, Record},
    anx7625::*,
};

static LOGGER: SimpleLogger = SimpleLogger;

struct SimpleLogger;

impl ::log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            print!("[{}]: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

struct I2C;
impl embedded_hal::blocking::i2c::Write for I2C {
    type Error = core::convert::Infallible;

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        println!("I2C write: 0x{:0x} {:?}", address, bytes);
        Ok(())
    }
}

impl embedded_hal::blocking::i2c::Read for I2C {
    type Error = core::convert::Infallible;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        println!("I2C read: 0x{:0x} {:?}", address, buffer);
        buffer[0] = 42;
        Ok(())
    }
}

struct Delay;
impl embedded_hal::blocking::delay::DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64))
    }
}

struct Pin;
impl embedded_hal::digital::v2::OutputPin for Pin {
    type Error = core::convert::Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        println!("Set pin low");
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        println!("Set pin high");
        Ok(())
    }
}

fn main() {
    ::log::set_logger(&LOGGER)
        .map(|()| ::log::set_max_level(::log::LevelFilter::Trace))
        .unwrap();

    let mut bus = I2C;
    let mut delay = Delay;
    let video_on = Pin;
    let video_rst = Pin;
    let otg_on = Pin;
    let mut anx = Anx7625::new(video_on, video_rst, otg_on);
    let res = anx.init(&mut bus, &mut delay);
    println!("Res: {:?}", res);
}
