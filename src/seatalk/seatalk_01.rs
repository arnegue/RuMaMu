use super::seatalk::{ParseError, SeatalkMessage, DATA_BYTES, MAX_SEATALK_LENGTH};
use bimap::BiMap;
use core::marker::Sized;
use core::result::Result;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Equipment {
    CourseComputer400G,
    ST60Tridata,
    ST60TridataPlus,
    ST60Log,
    ST80Masterview,
    ST80MaxiDisplay,
    SmartControllerRemoteControlHandset,
}

pub struct Sentence01 {
    pub equipment: Equipment,
    pub equipment_map: BiMap<[u8; 6], Equipment>
}

/*
01  05  XX XX XX XX XX XX  Equipment ID, sent at power on, reported examples:
01  05  00 00 00 60 01 00  Course Computer 400G
01  05  04 BA 20 28 01 00  ST60 Tridata
01  05  70 99 10 28 01 00  ST60 Log
01  05  F3 18 00 26 0F 06  ST80 Masterview
01  05  FA 03 00 30 07 03  ST80 Maxi Display
01  05  FF FF FF D0 00 00  Smart Controller Remote Control Handset
*/
impl SeatalkMessage for Sentence01 {
    const ID: u8 = 0x01;
    const LENGTH: usize = 8;

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

        let equipment_map = Self::create_equipment_map();
        // Get slice
        let equipment_id: [u8; 6] = buffer[2..Self::LENGTH]
            .try_into()
            .map_err(|_| ParseError::WrongLength)?;

        // Try to extract enum
        let equipment_result = equipment_map.get_by_left(&equipment_id);

        match equipment_result {
            Some(equipment) => Ok(Sentence01 {
                equipment: *equipment,
                equipment_map,
            }),
            None => Err(ParseError::UnexpectedData),
        }
    }

    fn generate_seatalk_data(&self) -> [u8; MAX_SEATALK_LENGTH] {
        let mut return_buffer = [0u8; MAX_SEATALK_LENGTH];
        return_buffer[0] = Self::ID;
        return_buffer[1] = (Self::LENGTH - DATA_BYTES) as u8;

        let equipment_bytes_result = self.equipment_map.get_by_right(&self.equipment);
        match equipment_bytes_result {
            Some(equipment_bytes) => {
                return_buffer[2..equipment_bytes.len() + 2].copy_from_slice(equipment_bytes);
            }
            None => {
                // TODO return 0 bytes?
            }
        }

        return_buffer
    }
}

impl Sentence01 {
    pub fn create_equipment_map() -> BiMap<[u8; 6], Equipment>  {        
        let mut equipment_map = BiMap::new();
        equipment_map.insert([0x00, 0x00, 0x00, 0x60, 0x01, 0x00], Equipment::CourseComputer400G);
        equipment_map.insert([0x04, 0xBA, 0x20, 0x28, 0x01, 0x00], Equipment::ST60Tridata);
        equipment_map.insert([0x87, 0x72, 0x25, 0x28, 0x01, 0x00], Equipment::ST60TridataPlus);
        equipment_map.insert([0x70, 0x99, 0x10, 0x28, 0x01, 0x00], Equipment::ST60Log);
        equipment_map.insert([0xF3, 0x18, 0x00, 0x26, 0x0F, 0x06], Equipment::ST80Masterview);
        equipment_map.insert([0xFA, 0x03, 0x00, 0x30, 0x07, 0x03], Equipment::ST80MaxiDisplay);
        equipment_map.insert([0xFF, 0xFF, 0xFF, 0xD0, 0x00, 0x00], Equipment::SmartControllerRemoteControlHandset);
        equipment_map
    }
}
