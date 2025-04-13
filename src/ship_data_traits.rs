// TODO more "generic" types for time, distances, distances/time, temperature and so on
// E.g. get better get rid of float?

pub trait WaterDepth {
    // Returns the water depth (below transducer) in centimeters
    fn get_depth_cm(&self) -> u16;
}

pub trait WaterTemperature {
    // Returns the temperature in degree celsius
    fn get_temperature_c(&self) -> f64;
}

pub trait SpeedThroughWater {
    // Returns the speed through_water in knots (TODO better some SI unit)
    fn get_speed_through_water_knots(&self) -> f64;
}
