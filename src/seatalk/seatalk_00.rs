use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use crate::ship_data_traits::WaterDepth;
use core::marker::Sized;
use core::result::Result;

pub struct Sentence00 {
    pub depth_below_transducer_cm: u16,
    pub anchor_alarm: bool,
    pub metric_display: bool,
    pub transducer_defect: bool,
    pub depth_alarm: bool,
    pub shallow_alarm: bool,
}

/*
 00  02  YZ  XX XX  Depth below transducer: XXXX/10 feet
                       Flags in Y: Y&8 = 8: Anchor Alarm is active
                                  Y&4 = 4: Metric display units or
                                           Fathom display units if followed by command 65
                                  Y&2 = 2: Used, unknown meaning
                      Flags in Z: Z&4 = 4: Transducer defective
                                  Z&2 = 2: Deep Alarm is active
                                  Z&1 = 1: Shallow Depth Alarm is active
                    Corresponding NMEA sentences: DPT, DBT
*/
impl SeatalkMessage for Sentence00 {
    const ID: u8 = 0x00;
    const LENGTH: usize = 5;

    fn parse_seatalk_data(
        buffer: [u8; MAX_SEATALK_LENGTH],
        message_length: usize,
    ) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        if buffer[0] != Self::ID {
            return Err(ParseError::WrongID);
        } else if message_length != Self::LENGTH {
            return Err(ParseError::WrongLength);
        }

        let depth_tenth_feet: u16 = ((buffer[4] as u16) << 8_u8) | buffer[3] as u16; // feet*10
        let depth_below_transducer_cm = (((depth_tenth_feet as u32) * 381_u32) / 125_u32) as u16; // Shortened from 3048/1000
        let anchor_alarm: bool = (buffer[2] & 128) != 0;
        let metric_display: bool = (buffer[2] & 64) != 0;
        let transducer_defect: bool = (buffer[2] & 4) != 0;
        let depth_alarm: bool = (buffer[2] & 2) != 0;
        let shallow_alarm: bool = (buffer[2] & 1) != 0;

        Ok(Sentence00 {
            depth_below_transducer_cm,
            anchor_alarm,
            metric_display,
            transducer_defect,
            depth_alarm,
            shallow_alarm,
        })
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;

        let mut flags: u8 = 0;
        flags |= if self.anchor_alarm { 0x80 } else { 0x00 };
        flags |= if self.metric_display { 0x40 } else { 0x00 };
        flags |= if self.transducer_defect { 0x04 } else { 0x00 };
        flags |= if self.depth_alarm { 0x02 } else { 0x00 };
        flags |= if self.shallow_alarm { 0x01 } else { 0x00 };
        return_buffer[2] = flags;

        let depth_tenth_feet: u16 = ((self.depth_below_transducer_cm as u32 * 125_u32) / 381_u32) as u16;
        return_buffer[3] = (depth_tenth_feet) as u8;
        return_buffer[4] = (depth_tenth_feet >> 8_u8) as u8;
        return_buffer
    }
}

impl WaterDepth for Sentence00 {
    fn get_depth_cm(&self) -> u16 {
        self.depth_below_transducer_cm
    }
}
