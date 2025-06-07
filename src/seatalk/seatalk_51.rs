use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use crate::helper::units::DMM;
use crate::ship_data_traits::Longitude;
use core::marker::Sized;
use core::result::Result;

#[derive(Debug, Clone)]
pub struct Sentence51 {
    pub longitude: DMM,
}

/*
51  Z2  XX  YY  YY  LON position: XX degrees, (YYYY & 0x7FFF)/100 minutes
                                          MSB of Y = YYYY & 0x8000 = East if set, West if cleared
                                          Z= 0xA or 0x0 (reported for Raystar 120 GPS), meaning unknown
                    Stable filtered position, for raw data use command 58
                    Corresponding NMEA sentences: RMC, GAA, GLL
*/
impl SeatalkMessage for Sentence51 {
    const ID: u8 = 0x51;
    const LENGTH: usize = 5;

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

        let degrees: u8 = buffer[2];
        let y: u16 = ((buffer[4] as u16) << 8_u8) | buffer[3] as u16;
        let minutes: f64 = (y & 0x7FFF) as f64 / 100.0;
        let is_east: bool = (y & 0x8000) != 0;

        let longitude: DMM = DMM {
            degrees,
            minutes,
            direction: if is_east { 'e' } else { 'w' },
        };

        Ok(Sentence51 { longitude })
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;

        let mut yyyy = (self.longitude.minutes * 100.0) as u16;
        if self.longitude.direction == 'e' {
            yyyy |= 0x8000;
        }

        return_buffer[2] = self.longitude.degrees;
        return_buffer[3] = (yyyy) as u8;
        return_buffer[4] = (yyyy >> 8_u8) as u8;
        return_buffer
    }
}

impl Longitude for Sentence51 {
    fn get_longitude(&self) -> DMM {
        self.longitude
    }
}
