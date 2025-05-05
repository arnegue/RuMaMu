use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use crate::ship_data_traits::SpeedThroughWater;
use core::marker::Sized;
use core::result::Result;

pub struct Sentence26 {
    pub speed_through_water_knots: f64,
    pub speed_valid: bool, // TODO other values? such as average speed and so on
}

/*
26  04  XX  XX  YY  YY DE  Speed through water:
                     XXXX/100 Knots, sensor 1, current speed, valid if D&4=4
                     YYYY/100 Knots, average speed (trip/time) if D&8=0
                              or data from sensor 2 if D&8=8
                     E&1=1: Average speed calculation stopped
                     E&2=2: Display value in MPH
*/
impl SeatalkMessage for Sentence26 {
    const ID: u8 = 0x26;
    const LENGTH: usize = 7;

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

        let speed_through_water_knots: f64 = (((buffer[3] as u16) << 8) | buffer[2] as u16) as f64 / 100.0;
        let speed_valid: bool = (buffer[6] & 0x40) == 0x40;
        Ok(Sentence26 {
            speed_through_water_knots,
            speed_valid,
        })
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;

        let speed_uint: u16 = (self.speed_through_water_knots * 100.0) as u16;

        return_buffer[2] = (speed_uint & 0xFF) as u8;
        return_buffer[3] = (speed_uint >> 8) as u8;
        return_buffer[6] |= if self.speed_valid { 0x40 } else { 0x40 };
        return_buffer
    }
}

impl SpeedThroughWater for Sentence26 {
    fn get_speed_through_water_knots(&self) -> f64 {
        self.speed_through_water_knots as f64
    }
}
