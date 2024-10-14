#[cfg(test)]
mod tests {
    use rumamu::{
        seatalk::{
            seatalk::{ParseError, SeatalkMessage},
            seatalk_00::{Sentence00, SEATALK_00_SIZE}
        },
        ship_data_traits::WaterDepth,
    };

    #[test]
    fn test_seatalk_parsing() {
        // Test normal parsing
        let expected_result = 2228; // 2228,88 cm
        let data = [0x00, 0x02, 0x00, 0xDB, 0x02]; // 0x02DB -> 731 -> 73.1 feet -> 2228.08 cm
        let result = Sentence00::parse_seatalk_data(data);

        match result {
            Ok(sentence00) => {
                assert_eq!(sentence00.get_depth_cm(), expected_result)
            }
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_seatalk_creation() {
        let expected_data = [0x00, 0x02, 0x00, 0xDA, 0x02]; // DA instead of DB, because of conversion error (we cant fit a float (2208.08 cm) there)

        let seatalk_sentence = Sentence00 {
            depth_cm: 2228,
            anchor_alarm: false,
            metric_display: false,
            transducer_defect: false,
            depth_alarm: false,
            shallow_alarm: false,
        };

        let actual_data = seatalk_sentence.generate_seatalk_data();
        assert_eq!(expected_data, actual_data);
    }

    #[test]
    fn test_message_id() {
        // Tests parsing a message with a wrong message ID
        let wrong_id: u8 = Sentence00::ID + 1;
        let test_buffer: [u8; SEATALK_00_SIZE] = [wrong_id; SEATALK_00_SIZE];
        let result: Result<Sentence00, ParseError> = Sentence00::parse_seatalk_data(test_buffer);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
