use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use core::marker::Sized;
use core::result::Result;

pub struct Sentence30 {
    pub intensity: u8,
}

/*
30  00  0X      Set lamp Intensity; X=0: L0, X=4: L1, X=8: L2, X=C: L3
                    (only sent once when setting the lamp intensity) 
*/
impl SeatalkMessage for Sentence30 {
    const ID: u8 = 0x30;
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

        Ok(Sentence30 {
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
