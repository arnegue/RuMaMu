use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use core::marker::Sized;
use core::result::Result;

pub struct Sentence80 {
    pub intensity: u8,
}

/*
80  00  0X      Set Lamp Intensity: X=0 off, X=4: 1, X=8: 2, X=C: 3
*/
impl SeatalkMessage for Sentence80 {
    const ID: u8 = 0x80;
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
        }

        let intensity = buffer[2] & 0x0F;

        Ok(Sentence80 {
            intensity
        })
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;
        return_buffer[2] = self.intensity;

        return_buffer
    }
}
