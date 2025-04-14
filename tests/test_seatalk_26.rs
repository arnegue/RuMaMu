#[cfg(test)]
mod tests_seatalk26 {
    use rumamu::{
        seatalk::{
            seatalk::{ParseError, SeatalkMessage, MAX_SEATALK_LENGTH},
            seatalk_26::Sentence26,
        },
        ship_data_traits::SpeedThroughWater,
    };

    const TEST_DATA: [u8; 7] = [0x26, 0x04, 0x07, 0x02, 0x00, 0x00, 0x40]; // 0x07 0x02 -> 0x0207 -> 519 -> 5.19 knots

    #[test]
    fn test_seatalk_parsing() {
        // Test normal parsing
        let expected_depth = 5.19;
        let expected_validity = true;
        let mut test_buffer = [0u8; MAX_SEATALK_LENGTH];

        test_buffer[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let result: Result<Sentence26, ParseError> =
            Sentence26::parse_seatalk_data(test_buffer, Sentence26::LENGTH);
        match result {
            Ok(sentence26) => {
                assert_eq!(sentence26.get_speed_through_water_knots(), expected_depth);
                assert_eq!(sentence26.speed_valid, expected_validity);
            }
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_seatalk_creation() {
        let mut expected_data = [0u8; MAX_SEATALK_LENGTH];

        expected_data[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let seatalk_sentence = Sentence26 {
            speed_knots: 5.19,
            speed_valid: true
        };

        let actual_data = seatalk_sentence.generate_seatalk_data();

        assert!(expected_data.iter().eq(actual_data.iter()));
    }

    #[test]
    fn test_message_length() {
        // Tests parsing a message with a wrong length
        let wrong_length: usize = Sentence26::LENGTH + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [Sentence26::ID; MAX_SEATALK_LENGTH];
        let result: Result<Sentence26, ParseError> =
            Sentence26::parse_seatalk_data(test_buffer, wrong_length);
        match result {
            Err(ParseError::WrongLength) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongLength"),
        }
    }

    #[test]
    fn test_message_id() {
        // Tests parsing a message with a wrong message ID
        let wrong_id: u8 = Sentence26::ID + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [wrong_id; MAX_SEATALK_LENGTH];
        let result: Result<Sentence26, ParseError> = Sentence26::parse_seatalk_data(test_buffer, 5);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
