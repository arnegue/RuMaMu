use crate::ship_data_traits::WaterDepth;


pub struct Sentence00 ;

impl WaterDepth for Sentence00 {
    fn get_depth_cm(& self) -> u16 {
        0
    }
}
