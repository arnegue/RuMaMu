use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use crate::ship_data_traits::SpeedOverGround;
use core::marker::Sized;
use core::result::Result;

pub struct Sentence52 {
    pub speed_over_ground: f64,
}

/*
52  01  XX  XX  Speed over Ground: XXXX/10 Knots
                Corresponding NMEA sentences: RMC, VTG
*/
impl SeatalkMessage for Sentence52 {
    const ID: u8 = 0x52;
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

        let speed_over_ground: f64 = (((buffer[3] as u16) << 8) | buffer[2] as u16) as f64 / 10.0;

        Ok(Sentence52 { speed_over_ground })
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;

        let speed_uint: u16 = (self.speed_over_ground * 10.0) as u16;

        return_buffer[2] = (speed_uint & 0xFF) as u8;
        return_buffer[3] = (speed_uint >> 8) as u8;
        return_buffer
    }
}

impl SpeedOverGround for Sentence52 {
    fn get_speed_over_ground_knots(&self) -> f64 {
        self.speed_over_ground
    }
}
