#[cfg(test)]
mod tests_seatalk20 {
    use rumamu::{
        seatalk::{
            seatalk::{ParseError, SeatalkMessage, MAX_SEATALK_LENGTH},
            seatalk_20::Sentence20,
        },
        ship_data_traits::SpeedThroughWater,
    };

    const TEST_DATA: [u8; 4] = [0x20, 0x01, 0x53, 0x01]; // 0x53 0x01 -> 0x0153 -> 339 -> 33.9 knots

    #[test]
    fn test_seatalk_parsing() {
        // Test normal parsing
        let expected_result = 33.9;
        let mut test_buffer = [0u8; MAX_SEATALK_LENGTH];

        test_buffer[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let result: Result<Sentence20, ParseError> =
            Sentence20::parse_seatalk_data(test_buffer, Sentence20::LENGTH);
        match result {
            Ok(sentence20) => {
                assert_eq!(sentence20.get_speed_through_water_knots(), expected_result)
            }
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_seatalk_creation() {
        let mut expected_data = [0u8; MAX_SEATALK_LENGTH];

        expected_data[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let seatalk_sentence = Sentence20 {
            speed_knots: 33.9
        };

        let actual_data = seatalk_sentence.generate_seatalk_data();

        assert!(expected_data.iter().eq(actual_data.iter()));
    }

    #[test]
    fn test_message_length() {
        // Tests parsing a message with a wrong length
        let wrong_length: usize = Sentence20::LENGTH + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [Sentence20::ID; MAX_SEATALK_LENGTH];
        let result: Result<Sentence20, ParseError> =
            Sentence20::parse_seatalk_data(test_buffer, wrong_length);
        match result {
            Err(ParseError::WrongLength) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongLength"),
        }
    }

    #[test]
    fn test_message_id() {
        // Tests parsing a message with a wrong message ID
        let wrong_id: u8 = Sentence20::ID + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [wrong_id; MAX_SEATALK_LENGTH];
        let result: Result<Sentence20, ParseError> = Sentence20::parse_seatalk_data(test_buffer, 5);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
