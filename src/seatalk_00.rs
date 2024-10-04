use crate::seatalk::{ParseError, SeatalkMessage};
use crate::ship_data_traits::WaterDepth;

pub struct Sentence00 {
    pub depth_cm: u16,
    pub anchor_alarm: bool,
    pub metric_display: bool,
    pub transducer_defect: bool,
    pub depth_alarm: bool,
    pub shallow_alarm: bool,
}

impl SeatalkMessage for Sentence00 {
    const ID: u8 = 0;
    const LENGTH: usize = 5;

    fn parse_seatalk_data(buffer: [u8; 256], message_length: usize) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        if buffer[0] != Self::ID {
            return Err(ParseError::WrongID);
        } else if message_length != Self::LENGTH {
            return Err(ParseError::WrongLength);
        }

        let depth_foot: u16 = (((buffer[3] as u16) << 8_u8) | buffer[4] as u16) / 10;
        let depth_cm = unit_conversions::length::feet::to_centimetres(depth_foot.into());
        let anchor_alarm: bool = (buffer[2] & 128) != 0;
        let metric_display: bool = (buffer[2] & 64) != 0;
        let transducer_defect: bool = (buffer[2] & 4) != 0;
        let depth_alarm: bool = (buffer[2] & 2) != 0;
        let shallow_alarm: bool = (buffer[2] & 1) != 0;

        Ok(Sentence00 {
            depth_cm: depth_cm as u16, // TODO maybe use f64 generally?
            anchor_alarm,
            metric_display,
            transducer_defect,
            depth_alarm,
            shallow_alarm,
        })
    }
}

impl WaterDepth for Sentence00 {
    fn get_depth_cm(&self) -> u16 {
        self.depth_cm
    }
}
