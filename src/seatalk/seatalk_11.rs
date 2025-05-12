use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use crate::ship_data_traits::WindSpeed;
use core::marker::Sized;
use core::result::Result;

pub struct Sentence11 {
    pub apparent_wind_speed_deca_knots: u16,
    pub display_knots: bool,
}

/*
 11  01  XX  0Y  Apparent Wind Speed: (XX & 0x7F) + Y/10 Knots
                 Units flag: XX&0x80=0    => Display value in Knots
                             XX&0x80=0x80 => Display value in Meter/Second
                 Corresponding NMEA sentence: MWV
*/
impl SeatalkMessage for Sentence11 {
    const ID: u8 = 0x11;
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

        let apparent_wind_speed_deca_knots =
            (((buffer[2] as u16) & 0x7F) * 10) + (buffer[3] as u16);
        let display_knots = buffer[2] & 0x80 == 0x80;

        Ok(Sentence11 {
            apparent_wind_speed_deca_knots: apparent_wind_speed_deca_knots,
            display_knots,
        })
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;

        return_buffer[2] = (self.apparent_wind_speed_deca_knots / 10) as u8;
        if self.display_knots {
            return_buffer[2] = return_buffer[2] | 0x80;
        }
        return_buffer[3] = (self.apparent_wind_speed_deca_knots % 10) as u8;
        return_buffer
    }
}

impl WindSpeed for Sentence11 {
    fn get_wind_speed_knots(&self) -> f32 {
        (self.apparent_wind_speed_deca_knots as f32) / 10.0
    }
}
