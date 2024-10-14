use core::marker::Sized;
use core::result::Result;

pub enum ParseError {
    WrongID,
    WrongLength,
}

pub trait SeatalkMessage<const LENGTH: usize>  {
    const ID: u8; // Seatalk-ID of message (first byte)

    fn parse_seatalk_data(buffer: [u8; LENGTH]) -> Result<Self, ParseError>
    where
        Self: Sized; // Parses given buffer and sets internal values
    fn generate_seatalk_data(&self) -> [u8; LENGTH]; // Returns own representation in seatalk bytes
}


// fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
// where
//     D: DrawTarget<Color = COL>,
// {