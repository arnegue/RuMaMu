use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use crate::helper::units::DMM;
use crate::ship_data_traits::Latitude;
use core::marker::Sized;
use core::result::Result;

#[derive(Debug, Clone)]
pub struct Sentence50 {
    pub latitude: DMM,
}

/*
50  Z2  XX  YY  YY  LAT position: XX degrees, (YYYY & 0x7FFF)/100 minutes
                    MSB of Y = YYYY & 0x8000 = South if set, North if cleared
                    Z= 0xA or 0x0 (reported for Raystar 120 GPS), meaning unknown
                    Stable filtered position, for raw data use command 58
                    Corresponding NMEA sentences: RMC, GAA, GLL
*/
impl SeatalkMessage for Sentence50 {
    const ID: u8 = 0x50;
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
        let is_south: bool = (y & 0x8000) != 0;

        let latitude: DMM = DMM {
            degrees,
            minutes,
            direction: if is_south { 's' } else { 'n' },
        };

        Ok(Sentence50 { latitude })
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;

        let mut yyyy = (self.latitude.minutes * 100.0) as u16;
        if self.latitude.direction == 's' {
            yyyy |= 0x8000;
        }

        return_buffer[2] = self.latitude.degrees;
        return_buffer[3] = (yyyy) as u8;
        return_buffer[4] = (yyyy >> 8_u8) as u8;
        return_buffer
    }
}

impl Latitude for Sentence50 {
    fn get_latitude(&self) -> DMM {
        self.latitude
    }
}
