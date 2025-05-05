use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use crate::ship_data_traits::SpeedThroughWater;
use core::marker::Sized;
use core::result::Result;

pub struct Sentence20 {
    pub speed_through_water_knots: f32,
    // TODO isnt there some kind of sensor defective too?
}

/*
20  01  XX  XX  Speed through water: XXXX/10 Knots
                Corresponding NMEA sentence: VHW
*/
impl SeatalkMessage for Sentence20 {
    const ID: u8 = 0x20;
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

        let speed_through_water_knots: f32 = (((buffer[3] as u16) << 8) | buffer[2] as u16) as f32 / 10.0;

        Ok(Sentence20 { speed_through_water_knots })
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;

        let speed_uint: u16 = (self.speed_through_water_knots * 10.0) as u16;

        return_buffer[2] = (speed_uint & 0xFF) as u8;
        return_buffer[3] = (speed_uint >> 8) as u8;
        return_buffer
    }
}

impl SpeedThroughWater for Sentence20 {
    fn get_speed_through_water_knots(&self) -> f32 {
        self.speed_through_water_knots as f32
    }
}
