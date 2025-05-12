#[cfg(test)]
mod tests_seatalk10 {
    use rumamu::{
        seatalk::{
            seatalk::{ParseError, SeatalkMessage, MAX_SEATALK_LENGTH},
            seatalk_10::Sentence10,
        },
        ship_data_traits::WindAngle,
    };

    const TEST_DATA: [u8; 4] = [0x10, 0x01, 0x01, 0x66]; //	0x0166 -> 358 -> 179°
    const EXPECTED_RESULT: u16 = 179;

    #[test]
    fn test_seatalk_parsing() {
        // Test normal parsing
        let mut test_buffer = [0u8; MAX_SEATALK_LENGTH];

        test_buffer[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let result: Result<Sentence10, ParseError> =
            Sentence10::parse_seatalk_data(test_buffer, Sentence10::LENGTH);
        match result {
            Ok(sentence10) => {
                assert_eq!(sentence10.get_wind_angle_degree(), EXPECTED_RESULT)
            }
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_seatalk_creation() {
        let mut expected_data = [0u8; MAX_SEATALK_LENGTH];

        expected_data[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let seatalk_sentence = Sentence10 {
            apparent_wind_angle_rob_degree: 179,
        };

        let actual_data = seatalk_sentence.generate_seatalk_data();

        assert!(expected_data.iter().eq(actual_data.iter()));
    }

    #[test]
    fn test_message_length() {
        // Tests parsing a message with a wrong length
        let wrong_length: usize = Sentence10::LENGTH + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [Sentence10::ID; MAX_SEATALK_LENGTH];
        let result: Result<Sentence10, ParseError> =
            Sentence10::parse_seatalk_data(test_buffer, wrong_length);
        match result {
            Err(ParseError::WrongLength) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongLength"),
        }
    }

    #[test]
    fn test_message_id() {
        // Tests parsing a message with a wrong message ID
        let wrong_id: u8 = Sentence10::ID + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [wrong_id; MAX_SEATALK_LENGTH];
        let result: Result<Sentence10, ParseError> = Sentence10::parse_seatalk_data(test_buffer, 5);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
