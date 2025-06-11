#[cfg(test)]
mod tests_seatalk54 {
    use rumamu::{
        helper::units::Time as UnitTime,
        seatalk::{
            seatalk::{ParseError, SeatalkMessage, MAX_SEATALK_LENGTH},
            seatalk_54::Sentence54,
        },
        ship_data_traits::Time,
    };

    const TEST_DATA: [u8; 4] = [0x54, 0xB1, 0xC3, 0x17]; //	0xC3B (RST) -> >>6 -> 48th minute | RST&0x3F -> 59th second | 0x17 -> 23th hour
    const EXPECTED_RESULT: UnitTime = UnitTime {
        hour: 23,
        minute: 48,
        second: 59,
    };

    #[test]
    fn test_seatalk_parsing() {
        // Test normal parsing
        let mut test_buffer = [0u8; MAX_SEATALK_LENGTH];

        test_buffer[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let result: Result<Sentence54, ParseError> =
            Sentence54::parse_seatalk_data(test_buffer, Sentence54::LENGTH);
        match result {
            Ok(sentence54) => {
                assert_eq!(sentence54.get_time(), EXPECTED_RESULT)
            }
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_seatalk_creation() {
        let mut expected_data = [0u8; MAX_SEATALK_LENGTH];

        expected_data[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let seatalk_sentence = Sentence54 {
            time: EXPECTED_RESULT,
        };

        let actual_data = seatalk_sentence.generate_seatalk_data();

        assert!(expected_data.iter().eq(actual_data.iter()));
    }

    #[test]
    fn test_unexpected_data() {
        // Tests parsing a message with a wrong data
        let mut test_buffer: [u8; MAX_SEATALK_LENGTH] = [0; MAX_SEATALK_LENGTH];
        test_buffer[0] = Sentence54::ID;
        test_buffer[1] = Sentence54::LENGTH as u8;
        test_buffer[3] = 120; // Set hour-byte to something > 60
                              // Keep the rest 0

        let result: Result<Sentence54, ParseError> =
            Sentence54::parse_seatalk_data(test_buffer, Sentence54::LENGTH);
        match result {
            Err(ParseError::UnexpectedData) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::UnexpectedData"),
        }
    }

    #[test]
    fn test_message_length() {
        // Tests parsing a message with a wrong length
        let wrong_length: usize = Sentence54::LENGTH + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [Sentence54::ID; MAX_SEATALK_LENGTH];
        let result: Result<Sentence54, ParseError> =
            Sentence54::parse_seatalk_data(test_buffer, wrong_length);
        match result {
            Err(ParseError::WrongLength) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongLength"),
        }
    }

    #[test]
    fn test_message_id() {
        // Tests parsing a message with a wrong message ID
        let wrong_id: u8 = Sentence54::ID + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [wrong_id; MAX_SEATALK_LENGTH];
        let result: Result<Sentence54, ParseError> = Sentence54::parse_seatalk_data(test_buffer, 5);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
