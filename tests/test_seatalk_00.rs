pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use rumamu::{
        seatalk::{ParseError, SeatalkMessage},
        seatalk_00::Sentence00,
    };

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_seatalk_parsing() {
        //stuff
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
        let result: Result<Sentence00, ParseError> = Sentence00::parse_seatalk_data(test_buffer, wrong_length);
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
        let result: Result<Sentence00, ParseError> =Sentence00::parse_seatalk_data(test_buffer, 5);
        match result {
            Err(ParseError::WrongID) => {} // Match the specific error variant
            _ => panic!("Expected ParseError::WrongID"),
        }
    }
}
