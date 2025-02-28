use core::marker::Sized;
use core::result::Result;

pub const MAX_SEATALK_LENGTH: usize = 25;

/// Error types which may occur when parsing received buffer
pub enum ParseError {
    WrongID,     // Message ID is wrong
    WrongLength, // Actual received message_length is not expected
}

/// Default trait for Seatalk messages
pub trait SeatalkMessage {
    const ID: u8; // Seatalk-ID of message (first byte)
    const LENGTH: usize; // Maximum length overall

    /// Tries to parse the received message and will return an instance of a SeatalkMessage or a ParseError
    fn parse_seatalk_data(
        buffer: [u8; MAX_SEATALK_LENGTH],
        message_length: usize,
    ) -> Result<Self, ParseError>
    where
        Self: Sized; // Parses given buffer and sets internal values
    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH]; // Returns own representation in seatalk bytes
}
