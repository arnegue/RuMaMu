use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use core::marker::Sized;
use core::result::Result;

pub struct Sentence36 {}

/*
36  00  01      Cancel MOB (Man Over Board) condition
*/
impl SeatalkMessage for Sentence36 {
    const ID: u8 = 0x36;
    const LENGTH: usize = 3;

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
        } else if (buffer[1] != 0x00) || (buffer[2] != 0x01) {
            return Err(ParseError::UnexpectedData);
        }

        Ok(Sentence36 {})
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;
        return_buffer[2] = 0x01;

        return_buffer
    }
}
