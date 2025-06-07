#[cfg(test)]
mod tests_seatalk51 {
    use rumamu::{
        helper::units::DMM,
        seatalk::{
            seatalk::{ParseError, SeatalkMessage, MAX_SEATALK_LENGTH},
            seatalk_51::Sentence51,
        },
        ship_data_traits::Longitude,
    };

    const TEST_DATA: [u8; 5] = [0x51, 0x02, 0x08, 0x05, 0x8B]; // 0x08 -> 8° (north) | 0x05 0x8B -> 0x8B05 -> B05 (8->East) -> 2821 -> 28.21 minutes
    const EXPECTED_RESULT: DMM = DMM {
        degrees: 8,
        minutes: 28.21,
        direction: 'e',
    };

    // Helper to compare two DMMs (with float comparison)
    fn cmp_dmm(first: DMM, second: DMM) {
        assert_eq!(first.degrees, second.degrees);
        assert!((first.minutes - second.minutes).abs() <= 0.001);
        assert_eq!(first.direction, second.direction);
    }

    #[test]
    fn test_seatalk_parsing() {
        // Test normal parsing
        let mut test_buffer = [0u8; MAX_SEATALK_LENGTH];

        test_buffer[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let result: Result<Sentence51, ParseError> =
            Sentence51::parse_seatalk_data(test_buffer, Sentence51::LENGTH);
        match result {
            Ok(sentence51) => {
                cmp_dmm(sentence51.get_longitude(), EXPECTED_RESULT);
            }
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_seatalk_creation() {
        let mut expected_data = [0u8; MAX_SEATALK_LENGTH];

        expected_data[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let seatalk_sentence = Sentence51 {
            longitude: EXPECTED_RESULT
        };

        let actual_data = seatalk_sentence.generate_seatalk_data();

        assert!(expected_data.iter().eq(actual_data.iter()));
    }

    #[test]
    fn test_message_length() {
        // Tests parsing a message with a wrong length
        let wrong_length: usize = Sentence51::LENGTH + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [Sentence51::ID; MAX_SEATALK_LENGTH];
        let result: Result<Sentence51, ParseError> =
            Sentence51::parse_seatalk_data(test_buffer, wrong_length);
        match result {
            Err(ParseError::WrongLength) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongLength"),
        }
    }

    #[test]
    fn test_message_id() {
        // Tests parsing a message with a wrong message ID
        let wrong_id: u8 = Sentence51::ID + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [wrong_id; MAX_SEATALK_LENGTH];
        let result: Result<Sentence51, ParseError> = Sentence51::parse_seatalk_data(test_buffer, 5);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
