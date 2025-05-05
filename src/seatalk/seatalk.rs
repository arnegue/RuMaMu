use core::marker::Sized;
use core::result::Result;

pub const MAX_SEATALK_LENGTH: usize = 25; // Maximum length of a seatalk message
pub const DATA_BYTES: usize = 3; // "Minimum" length. First byte: ID, Second: Length, 3rd at least one Data-Byte

/// Error types which may occur when parsing received buffer
pub enum ParseError {
    WrongID,        // Message ID is wrong
    WrongLength,    // Actual received message_length is not expected
    UnexpectedData, // The Data in the buffer are not expected (e.g. unknown Enum value)
}

/// Default trait for Seatalk messages
pub trait SeatalkMessage {
    const ID: u8; // Seatalk-ID of message (first byte)
    const LENGTH: usize; // Length of whole seatalk message (including ID-byte and so on)

    /// Tries to parse the received message and will return an instance of a SeatalkMessage or a ParseError
    fn parse_seatalk_data(
        buffer: [u8; MAX_SEATALK_LENGTH], // Buffer containing every byte (including ID byte)
        message_length: usize,            // Length of buffer (may vary)
    ) -> Result<Self, ParseError>
    where
        Self: Sized; // Parses given buffer and sets internal values

    // Returns own representation in seatalk bytes
    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH];
}

// TODOs for some re-occurring code:
//  bytehandling
//  ID-Check
//  Length-Check
