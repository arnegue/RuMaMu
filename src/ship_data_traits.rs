pub trait WaterDepth {
    // Returns the water depth (below transducer) in centimeters
    fn get_depth_cm(&self) -> u16;
}

pub trait WaterTemperature {
    // Returns the temperature in degree celsius
    fn get_temperature_c(&self) -> u8; // TODO u8? Maybe float/double (better avoid them for non-float uC
}
