pub trait WaterDepth {
    // Returns the water depth (below transducer) in centimeters
    fn get_depth_cm(&self) -> u16;
}

pub trait WaterTemperature {
    // Returns the temperature in degree celsius
    fn get_temperature_c(&self) -> f64;
}
