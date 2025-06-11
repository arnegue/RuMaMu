#[cfg(test)]
mod tests_seatalk56 {
    use rumamu::{
        helper::units::Date as UnitDate,
        seatalk::{
            seatalk::{ParseError, SeatalkMessage, MAX_SEATALK_LENGTH},
            seatalk_56::Sentence56,
        },
        ship_data_traits::Date,
    };

    const TEST_DATA: [u8; 4] = [0x56, 0xC1, 0x18, 0x14]; //	0x0C -> 12 -> December, 0x18 -> 24th day, 0x14 -> 20 -> 2020 year
    const EXPECTED_RESULT: UnitDate = UnitDate {
        year: 2020,
        month: 12,
        day: 24,
    };

    #[test]
    fn test_seatalk_parsing() {
        // Test normal parsing
        let mut test_buffer = [0u8; MAX_SEATALK_LENGTH];

        test_buffer[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let result: Result<Sentence56, ParseError> =
            Sentence56::parse_seatalk_data(test_buffer, Sentence56::LENGTH);
        match result {
            Ok(sentence56) => {
                assert_eq!(sentence56.get_date(), EXPECTED_RESULT)
            }
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_seatalk_creation() {
        let mut expected_data = [0u8; MAX_SEATALK_LENGTH];

        expected_data[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let seatalk_sentence = Sentence56 {
            date: EXPECTED_RESULT,
        };

        let actual_data = seatalk_sentence.generate_seatalk_data();

        assert!(expected_data.iter().eq(actual_data.iter()));
    }

    #[test]
    fn test_unexpected_data() {
        // Tests parsing a message with a wrong data
        let mut test_buffer: [u8; MAX_SEATALK_LENGTH] = [0; MAX_SEATALK_LENGTH];
        test_buffer[0] = Sentence56::ID;
        test_buffer[1] = Sentence56::LENGTH as u8;
        // Keep the rest 0

        let result: Result<Sentence56, ParseError> =
        Sentence56::parse_seatalk_data(test_buffer, Sentence56::LENGTH);
        match result {
            Err(ParseError::UnexpectedData) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::UnexpectedData"),
        }
    }

    #[test]
    fn test_message_length() {
        // Tests parsing a message with a wrong length
        let wrong_length: usize = Sentence56::LENGTH + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [Sentence56::ID; MAX_SEATALK_LENGTH];
        let result: Result<Sentence56, ParseError> =
            Sentence56::parse_seatalk_data(test_buffer, wrong_length);
        match result {
            Err(ParseError::WrongLength) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongLength"),
        }
    }

    #[test]
    fn test_message_id() {
        // Tests parsing a message with a wrong message ID
        let wrong_id: u8 = Sentence56::ID + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [wrong_id; MAX_SEATALK_LENGTH];
        let result: Result<Sentence56, ParseError> = Sentence56::parse_seatalk_data(test_buffer, 5);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
