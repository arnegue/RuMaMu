#[cfg(test)]
mod tests_seatalk00 {
    use rumamu::{
        seatalk::{
            seatalk::{ParseError, SeatalkMessage, MAX_SEATALK_LENGTH},
            seatalk_00::Sentence00,
        },
        ship_data_traits::WaterDepth,
    };

    #[test]
    fn test_seatalk_parsing() {
        // Test normal parsing
        let expected_result = 2228; // 2228,88 cm
        let mut test_buffer = [0u8; MAX_SEATALK_LENGTH];
        let data = [0x00, 0x02, 0x00, 0xDB, 0x02]; // 0x02DB -> 731 -> 73.1 feet -> 2228.08 cm
        test_buffer[..data.len()].copy_from_slice(&data);

        let result: Result<Sentence00, ParseError> =
            Sentence00::parse_seatalk_data(test_buffer, Sentence00::LENGTH);
        match result {
            Ok(sentence00) => {
                assert_eq!(sentence00.get_depth_cm(), expected_result)
            }
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_seatalk_creation() {
        let mut expected_data = [0u8; MAX_SEATALK_LENGTH];
        let data = [0x00, 0x02, 0x00, 0xDA, 0x02]; // DA instead of DB, because of conversion error (we cant fit a float (2208.08 cm) there)
        expected_data[..data.len()].copy_from_slice(&data);

        let seatalk_sentence = Sentence00 {
            depth_cm: 2228,
            anchor_alarm: false,
            metric_display: false,
            transducer_defect: false,
            depth_alarm: false,
            shallow_alarm: false,
        };

        let actual_data = seatalk_sentence.generate_seatalk_data();
        assert!(expected_data.iter().eq(actual_data.iter()));
    }

    #[test]
    fn test_message_length() {
        // Tests parsing a message with a wrong length
        let wrong_length: usize = Sentence00::LENGTH + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [Sentence00::ID; MAX_SEATALK_LENGTH];
        let result: Result<Sentence00, ParseError> =
            Sentence00::parse_seatalk_data(test_buffer, wrong_length);
        match result {
            Err(ParseError::WrongLength) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongLength"),
        }
    }

    #[test]
    fn test_message_id() {
        // Tests parsing a message with a wrong message ID
        let wrong_id: u8 = Sentence00::ID + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [wrong_id; MAX_SEATALK_LENGTH];
        let result: Result<Sentence00, ParseError> = Sentence00::parse_seatalk_data(test_buffer, 5);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
