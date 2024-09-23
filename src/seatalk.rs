pub trait SeatalkMessage {
    const ID: u8; // Seatalk-ID of message (first byte)
    const LENGTH: usize; // Maximum length overall
    fn parse_seatalk_data(&self, buffer: [u8; 256]) -> Self; // Parses given buffer and sets internal values // TODO result (error)
    //fn generate_seatalk_data(&self) -> [u8; Self::LENGTH]; // TODo
}
