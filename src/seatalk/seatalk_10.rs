use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use crate::ship_data_traits::WindAngle;
use core::marker::Sized;
use core::result::Result;

pub struct Sentence10 {
    pub apparent_wind_angle_rob_degree: u16, // ROB = Right of bow
}

/*
10  01  XX  YY  Apparent Wind Angle: XXYY/2 degrees right of bow
                Used for autopilots Vane Mode (WindTrim)
                Corresponding NMEA sentence: MWV
*/
impl SeatalkMessage for Sentence10 {
    const ID: u8 = 0x10;
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

        let apparent_wind_angle_rob_degree = ((buffer[2] as u16) << 8_u8) | buffer[3] as u16;
        let apparent_wind_angle_rob_degree = apparent_wind_angle_rob_degree >> 1;

        Ok(Sentence10 {
            apparent_wind_angle_rob_degree,
        })
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;

        let apparent_wind_angle_rob_degree = self.apparent_wind_angle_rob_degree << 1;
        return_buffer[2] = (apparent_wind_angle_rob_degree >> 8) as u8;
        return_buffer[3] = (apparent_wind_angle_rob_degree & 0xFF) as u8;

        return_buffer
    }
}

impl WindAngle for Sentence10 {
    fn get_wind_angle_degree(&self) -> u16 {
        self.apparent_wind_angle_rob_degree
    }
}
