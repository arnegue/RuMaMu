#[cfg(test)]
mod tests_seatalk27 {
    use rumamu::{
        seatalk::{
            seatalk::{ParseError, SeatalkMessage, MAX_SEATALK_LENGTH},
            seatalk_27::Sentence27,
        },
        ship_data_traits::WaterTemperature,
    };

    const TEST_DATA: [u8; 4] =  [0x27, 0x01, 0xF2, 0x01]; // 

    #[test]
    fn test_seatalk_parsing() {
        // Test normal parsing
        let expected_result = 39.8;
        let mut test_buffer = [0u8; MAX_SEATALK_LENGTH];

        test_buffer[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let result: Result<Sentence27, ParseError> =
            Sentence27::parse_seatalk_data(test_buffer, Sentence27::LENGTH);
        match result {
            Ok(sentence27) => {
                assert_eq!(sentence27.get_temperature_c(), expected_result)
            }
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_seatalk_creation() {
        let mut expected_data = [0u8; MAX_SEATALK_LENGTH];

        expected_data[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let seatalk_sentence = Sentence27 {
            water_temperature_c: 39.8,
        };

        let actual_data = seatalk_sentence.generate_seatalk_data();

        assert!(expected_data.iter().eq(actual_data.iter()));
    }

    #[test]
    fn test_message_length() {
        // Tests parsing a message with a wrong length
        let wrong_length: usize = Sentence27::LENGTH + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [Sentence27::ID; MAX_SEATALK_LENGTH];
        let result: Result<Sentence27, ParseError> =
            Sentence27::parse_seatalk_data(test_buffer, wrong_length);
        match result {
            Err(ParseError::WrongLength) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongLength"),
        }
    }

    #[test]
    fn test_message_id() {
        // Tests parsing a message with a wrong message ID
        let wrong_id: u8 = Sentence27::ID + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [wrong_id; MAX_SEATALK_LENGTH];
        let result: Result<Sentence27, ParseError> = Sentence27::parse_seatalk_data(test_buffer, 5);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
