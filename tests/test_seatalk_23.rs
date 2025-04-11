#[cfg(test)]
mod tests {
    use rumamu::{
        seatalk::{
            seatalk::{ParseError, SeatalkMessage, MAX_SEATALK_LENGTH}, seatalk_23::Sentence23
        },
        ship_data_traits::WaterTemperature,
    };

    const TEST_DATA: [u8; 4] = [0x23, 0x41, 0x11, 0]; // 0x3E]; // 0x11 -> 17 -> 17 °C TODO fahrenheit

    #[test]
    fn test_seatalk_parsing() {
        //  (WaterTemperature1(temperature_c=17.2, sensor_defective=True),                                   bytes()),
        // Test normal parsing
        let expected_result = 17; // 2228,88 cm
        let mut test_buffer = [0u8; MAX_SEATALK_LENGTH];
        
        test_buffer[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let result: Result<Sentence23, ParseError> =
            Sentence23::parse_seatalk_data(test_buffer, Sentence23::LENGTH);
        match result {
            Ok(sentence23) => {
                assert_eq!(sentence23.get_temperature_c(), expected_result)
            }
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_seatalk_creation() {
        let mut expected_data = [0u8; MAX_SEATALK_LENGTH];

        expected_data[..TEST_DATA.len()].copy_from_slice(&TEST_DATA);

        let seatalk_sentence = Sentence23 {
            sensor_defective: true,
            temperature_c: 11
        };

        let actual_data = seatalk_sentence.generate_seatalk_data();
        assert_eq!(expected_data, actual_data);
    }

    #[test]
    fn test_message_length() {
        // Tests parsing a message with a wrong length
        let wrong_length: usize = Sentence23::LENGTH + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [Sentence23::ID; MAX_SEATALK_LENGTH];
        let result: Result<Sentence23, ParseError> =
            Sentence23::parse_seatalk_data(test_buffer, wrong_length);
        match result {
            Err(ParseError::WrongLength) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongLength"),
        }
    }

    #[test]
    fn test_message_id() {
        // Tests parsing a message with a wrong message ID
        let wrong_id: u8 = Sentence23::ID + 1;
        let test_buffer: [u8; MAX_SEATALK_LENGTH] = [wrong_id; MAX_SEATALK_LENGTH];
        let result: Result<Sentence23, ParseError> = Sentence23::parse_seatalk_data(test_buffer, 5);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
