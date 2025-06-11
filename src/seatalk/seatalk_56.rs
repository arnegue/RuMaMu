use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use crate::helper::units::Date as UnitDate;
use crate::ship_data_traits::Date;
use core::marker::Sized;
use core::result::Result;

pub struct Sentence56 {
    pub date: UnitDate,
}

const YEAR_OFFSET: u16 = 2000; // TODO correct?

/*
56  M1  DD  YY  Date: YY year, M month, DD day in month
                Corresponding NMEA sentence: RMC 
*/
impl SeatalkMessage for Sentence56 {
    const ID: u8 = 0x56;
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
        
        let month = buffer[1] >> 4;
        let day: u8 = buffer[2];
        let year: u16 = buffer[3] as u16 + YEAR_OFFSET;

        // Minor data validation
        if month < 1 || month > 12 || day < 1 || day > 31 {
            return Err(ParseError::UnexpectedData);
        } 

        Ok(Sentence56 {
            date: UnitDate {
                year,
                month,
                day
            }
        })
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (self.date.month << 4) | (Self::LENGTH - DATA_BYTES) as u8;
        return_buffer[2] = self.date.day;
        return_buffer[3] = (self.date.year - YEAR_OFFSET) as u8;
        return_buffer
    }
}

impl Date for Sentence56 {
    fn get_date(&self) -> UnitDate {
        self.date
    }
}
