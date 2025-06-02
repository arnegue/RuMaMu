use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use crate::ship_data_traits::WaterTemperature;
use crate::helper::unit_converter::celsius_to_fahrenheit;
use core::marker::Sized;
use core::result::Result;

pub struct Sentence23 {
    pub sensor_defective: bool,
    pub water_temperature_c: u8,
    // 3rd byte is fahrenheit. Ignore it
}

/*
    23  Z1  XX  YY  Water temperature (ST50): XX deg Celsius, YY deg Fahrenheit
                 Flag Z&4: Sensor defective or not connected (Z=4)
                 Corresponding NMEA sentence: MTW
*/
impl SeatalkMessage for Sentence23 {
    const ID: u8 = 0x23;
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

        let water_temperature_c = buffer[2]; // Second byte is Celsius
                                       // Ignore 3rd byte (fahrenheit). When generating the temperature gets converted from celsius
        let sensor_defective = buffer[1] & 4 == 4;
        Ok(Sentence23 {
            water_temperature_c,
            sensor_defective,
        })
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;
        return_buffer[1] |= if self.sensor_defective { 0x40 } else { 0x00 };

        return_buffer[2] = self.water_temperature_c;
        return_buffer[3] = celsius_to_fahrenheit(self.water_temperature_c as f64) as u8; 
        return_buffer
    }
}

impl WaterTemperature for Sentence23 {
    fn get_temperature_c(&self) -> f64 {
        self.water_temperature_c as f64
    }
}
