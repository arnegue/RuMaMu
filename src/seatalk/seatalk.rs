use core::marker::Sized;
use core::result::Result;

pub enum ParseError {
    WrongID,
    WrongLength,
}

pub trait SeatalkMessage {
    const ID: u8; // Seatalk-ID of message (first byte)
    const LENGTH: usize; // Maximum length overall
    fn parse_seatalk_data(buffer: [u8; 256], message_length: usize) -> Result<Self, ParseError>
    where
        Self: Sized; // Parses given buffer and sets internal values
    fn generate_seatalk_data(&self) -> [u8; 256]; // Returns own representation in seatalk bytes
}
