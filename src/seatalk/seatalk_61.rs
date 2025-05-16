use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use core::marker::Sized;
use core::result::Result;

pub struct Sentence61 {}

/*
61  03  03 00 00 00  Issued by E-80 multifunction display at initialization
*/
impl SeatalkMessage for Sentence61 {
    const ID: u8 = 0x61;
    const LENGTH: usize = 6;

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
        } else if (buffer[2] != 0x03) || (buffer[3] != 0x00)|| (buffer[4] != 0x00)|| (buffer[5] != 0x00) {
            return Err(ParseError::UnexpectedData);
        }

        Ok(Sentence61 {})
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;
        return_buffer[2] = 0x03;
        return_buffer
    }
}
