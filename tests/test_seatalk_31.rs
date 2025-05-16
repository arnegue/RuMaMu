#[cfg(test)]
mod tests_seatalk31 {
    use rumamu::seatalk::{
        seatalk::{ParseError, SeatalkMessage, MAX_SEATALK_LENGTH},
        seatalk_31::Sentence31,
    };

    const TEST_DATA: [u8; 3] = [0x31, 0x00, 0x01];

    #[test]
    fn test_seatalk_parsing() {
        // Test normal parsing
        let mut test_buffer = [0u8; MAX_SEATALK_LENGTH];

        test_buffer[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let result: Result<Sentence31, ParseError> =
            Sentence31::parse_seatalk_data(test_buffer, Sentence31::LENGTH);
        match result {
            Ok(_) => {}
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_seatalk_creation() {
        let mut expected_data = [0u8; MAX_SEATALK_LENGTH];

        expected_data[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let seatalk_sentence = Sentence31 {};

        let actual_data = seatalk_sentence.generate_seatalk_data();

        assert!(expected_data.iter().eq(actual_data.iter()));
    }

    #[test]
    fn test_unexpected_data() {
        // Tests parsing a message with a wrong data
        let mut test_buffer: [u8; MAX_SEATALK_LENGTH] = [0; MAX_SEATALK_LENGTH];
        test_buffer[0] = Sentence31::ID;
        test_buffer[1] = 2;
        test_buffer[2] = 3;

        let result: Result<Sentence31, ParseError> =
            Sentence31::parse_seatalk_data(test_buffer, Sentence31::LENGTH);
        match result {
            Err(ParseError::UnexpectedData) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::UnexpectedData"),
        }
    }

    #[test]
    fn test_message_length() {
        // Tests parsing a message with a wrong length
        let wrong_length: usize = Sentence31::LENGTH + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [Sentence31::ID; MAX_SEATALK_LENGTH];
        let result: Result<Sentence31, ParseError> =
            Sentence31::parse_seatalk_data(test_buffer, wrong_length);
        match result {
            Err(ParseError::WrongLength) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongLength"),
        }
    }

    #[test]
    fn test_message_id() {
        // Tests parsing a message with a wrong message ID
        let wrong_id: u8 = Sentence31::ID + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [wrong_id; MAX_SEATALK_LENGTH];
        let result: Result<Sentence31, ParseError> = Sentence31::parse_seatalk_data(test_buffer, 5);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
