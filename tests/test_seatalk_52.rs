#[cfg(test)]
mod tests_seatalk52 {
    use rumamu::{
        seatalk::{
            seatalk::{ParseError, SeatalkMessage, MAX_SEATALK_LENGTH},
            seatalk_52::Sentence52,
        },
        ship_data_traits::SpeedOverGround,
    };

    const TEST_DATA: [u8; 4] = [0x52, 0x01, 0xD2, 0x04]; // 0x04D2 -> 1234 -> 123.4 Knots
    const EXPECTED_RESULT: f64 = 123.4;

    #[test]
    fn test_seatalk_parsing() {
        // Test normal parsing
        let mut test_buffer = [0u8; MAX_SEATALK_LENGTH];

        test_buffer[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let result: Result<Sentence52, ParseError> =
            Sentence52::parse_seatalk_data(test_buffer, Sentence52::LENGTH);
        match result {
            Ok(sentence52) => {
                assert_eq!(sentence52.get_speed_over_ground_knots(), EXPECTED_RESULT)
            }
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_seatalk_creation() {
        let mut expected_data = [0u8; MAX_SEATALK_LENGTH];

        expected_data[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let seatalk_sentence = Sentence52 {
            speed_over_ground: EXPECTED_RESULT,
        };

        let actual_data = seatalk_sentence.generate_seatalk_data();

        assert!(expected_data.iter().eq(actual_data.iter()));
    }

    #[test]
    fn test_message_length() {
        // Tests parsing a message with a wrong length
        let wrong_length: usize = Sentence52::LENGTH + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [Sentence52::ID; MAX_SEATALK_LENGTH];
        let result: Result<Sentence52, ParseError> =
            Sentence52::parse_seatalk_data(test_buffer, wrong_length);
        match result {
            Err(ParseError::WrongLength) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongLength"),
        }
    }

    #[test]
    fn test_message_id() {
        // Tests parsing a message with a wrong message ID
        let wrong_id: u8 = Sentence52::ID + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [wrong_id; MAX_SEATALK_LENGTH];
        let result: Result<Sentence52, ParseError> = Sentence52::parse_seatalk_data(test_buffer, 5);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
