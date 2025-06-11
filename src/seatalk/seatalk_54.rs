use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use crate::helper::units::Time as UnitTime;
use crate::ship_data_traits::Time;
use core::marker::Sized;
use core::result::Result;

pub struct Sentence54 {
    pub time: UnitTime,
}

/*
54  T1  RS  HH  GMT-time: HH hours,
                          6 MSBits of RST = minutes = (RS & 0xFC) / 4
                          6 LSBits of RST = seconds =  ST & 0x3F
                Corresponding NMEA sentences: RMC, GAA, BWR, BWC
*/
impl SeatalkMessage for Sentence54 {
    const ID: u8 = 0x54;
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

        let hour = buffer[3];
        let minute = buffer[2] >> 2;
        let second = (buffer[2] << 4) | (buffer[1] >> 4);

        if hour > 23 || minute > 59 || second > 59 {
            return Err(ParseError::UnexpectedData);
        }

        Ok(Sentence54 {
            time: UnitTime {
                hour,
                minute,
                second,
            },
        })
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;
        return_buffer[1] |= (self.time.second & 0xF) << 4;
        return_buffer[2] = self.time.minute << 2 | (self.time.second >> 4);
        return_buffer[3] = self.time.hour;

        return_buffer
    }
}

impl Time for Sentence54 {
    fn get_time(&self) -> UnitTime {
        self.time
    }
}
