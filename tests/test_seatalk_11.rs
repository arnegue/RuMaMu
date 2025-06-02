#[cfg(test)]
mod tests_seatalk11 {
    use rumamu::{
        seatalk::{
            seatalk::{ParseError, SeatalkMessage, MAX_SEATALK_LENGTH},
            seatalk_11::Sentence11,
        },
        ship_data_traits::WindSpeed,
    };

    const TEST_DATA: [u8; 4] = [0x11, 0x01, 0xFF, 0x03]; //	0xFF 0x03 -> 0x7F 0x03 -> 127 + (3/10) -> 127.3
    const EXPECTED_RESULT_KNOTS: f64 = 127.3;
    const EXPECTED_RESULT_DISPLAY: bool = true;

    #[test]
    fn test_seatalk_parsing() {
        // Test normal parsing
        let mut test_buffer = [0u8; MAX_SEATALK_LENGTH];

        test_buffer[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let result: Result<Sentence11, ParseError> =
            Sentence11::parse_seatalk_data(test_buffer, Sentence11::LENGTH);
        match result {
            Ok(sentence11) => {
                assert_eq!(sentence11.get_wind_speed_knots(), EXPECTED_RESULT_KNOTS);
                assert_eq!(sentence11.display_knots, EXPECTED_RESULT_DISPLAY);
            }
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_seatalk_creation() {
        let mut expected_data = [0u8; MAX_SEATALK_LENGTH];

        expected_data[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let seatalk_sentence = Sentence11 {
            apparent_wind_speed_deca_knots: (EXPECTED_RESULT_KNOTS * 10.0) as u16,
            display_knots: EXPECTED_RESULT_DISPLAY
        };

        let actual_data = seatalk_sentence.generate_seatalk_data();

        assert!(expected_data.iter().eq(actual_data.iter()));
    }

    #[test]
    fn test_message_length() {
        // Tests parsing a message with a wrong length
        let wrong_length: usize = Sentence11::LENGTH + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [Sentence11::ID; MAX_SEATALK_LENGTH];
        let result: Result<Sentence11, ParseError> =
            Sentence11::parse_seatalk_data(test_buffer, wrong_length);
        match result {
            Err(ParseError::WrongLength) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongLength"),
        }
    }

    #[test]
    fn test_message_id() {
        // Tests parsing a message with a wrong message ID
        let wrong_id: u8 = Sentence11::ID + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [wrong_id; MAX_SEATALK_LENGTH];
        let result: Result<Sentence11, ParseError> = Sentence11::parse_seatalk_data(test_buffer, 5);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
