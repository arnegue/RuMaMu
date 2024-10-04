pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use rumamu::{
        seatalk::{ParseError, SeatalkMessage},
        seatalk_00::Sentence00,
        ship_data_traits::WaterDepth,
    };

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_seatalk_parsing() {
        // Test normal parsing
        let expected_result = 2230;
        let mut test_buffer = [0u8; 256];
        let data = [0x00, 0x02, 0x00, 0xDB, 0x02];
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
        //stuff
    }

    #[test]
    fn test_message_length() {
        // Tests parsing a message with a wrong length
        let wrong_length: usize = Sentence00::LENGTH + 1;
        let test_buffer: [u8; 256] = [0; 256];
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
        let test_buffer: [u8; 256] = [wrong_id; 256];
        let result: Result<Sentence00, ParseError> = Sentence00::parse_seatalk_data(test_buffer, 5);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
