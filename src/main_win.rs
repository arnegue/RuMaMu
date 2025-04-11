use rumamu::seatalk::{
    seatalk::{ParseError, SeatalkMessage, MAX_SEATALK_LENGTH},
    seatalk_00::Sentence00
};
use rumamu::ship_data_traits::WaterDepth;

pub fn platform_specific_main() {
    let mut test_buffer = [0u8; MAX_SEATALK_LENGTH];
    let data = [0, 1, 2];
    test_buffer[..data.len()].copy_from_slice(&data);
    let result: Result<Sentence00, ParseError> =
        Sentence00::parse_seatalk_data(test_buffer, Sentence00::LENGTH);

    match result {
        Ok(sentence00) => {
            let depth =  sentence00.get_depth_cm();
            println!("Hello Windows. Depth: {depth}")
        }
        _ => panic!("Hello Windows, parsing failed"),
    }
}
