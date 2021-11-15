use embedded_hal::blocking::i2c::{Read as I2CRead, SevenBitAddress, Write as I2CWrite};

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
