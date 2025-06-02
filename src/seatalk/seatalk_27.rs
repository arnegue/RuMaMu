use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use crate::ship_data_traits::WaterTemperature;
use core::marker::Sized;
use core::result::Result;

pub struct Sentence27 {
    pub water_temperature_c: f64,  // TODO float maybe?
}

/*
    27  01  XX  XX  Water temperature: (XXXX-100)/10 deg Celsius
                 Corresponding NMEA sentence: MTW
*/
impl SeatalkMessage for Sentence27 {
    const ID: u8 = 0x27;
    const LENGTH: usize = 4;

    fn parse_seatalk_data(
        buffer: [u8; MAX_SEATALK_LENGTH],
        message_length: usize,
    ) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        if buffer[0] != Self::ID {
            return Err(ParseError::WrongID);
        } else if message_length != Self::LENGTH {
            return Err(ParseError::WrongLength);
        }

        let bytes = ((buffer[3] as u16) << 8) | buffer[2] as u16;
        let water_temperature_c = ((bytes - 100) as f64) / 10.0;
        Ok(Sentence27 {
            water_temperature_c,
        })
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;

        let bytes_value = ((self.water_temperature_c * 10.0) as u16) + 100;
        return_buffer[2] = (bytes_value & 0xFF) as u8;
        return_buffer[3] = (bytes_value >> 8) as u8;
        return_buffer
    }
}

impl WaterTemperature for Sentence27 {
    fn get_temperature_c(&self) -> f64 {
        self.water_temperature_c as f64
    }
}
