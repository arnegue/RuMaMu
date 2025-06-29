// TODO more "generic" types for time, distances, distances/time, temperature and so on
// E.g. get better get rid of float?

use crate::helper::units::DMM;
use crate::helper::units::Date as UnitDate;
use crate::helper::units::Time as UnitTime;

pub trait WaterDepth {
    // Returns the water depth (below transducer) in centimeters
    fn get_depth_cm(&self) -> u16;
}

pub trait WaterTemperature {
    // Returns the temperature in degree celsius
    fn get_temperature_c(&self) -> f64;
}

pub trait SpeedThroughWater {
    // Returns the speed through water in knots (TODO better use SI unit)
    fn get_speed_through_water_knots(&self) -> f64;
}

pub trait SpeedOverGround {
    // Returns the speed over ground in knots (TODO better use SI unit)
    fn get_speed_over_ground_knots(&self) -> f64;
}

pub trait WindSpeed {
    // Returns the wind speed in knots  (TODO better use SI unit)
    fn get_wind_speed_knots(&self) -> f64;
}

pub trait WindAngle {
    // Returns the wind angle. Depending on datagram the meaning of it may differ (true or magnetic heading, relative to bow, ...)
    fn get_wind_angle_degree(&self) -> u16;
}

pub trait CurrentMileage {
    // Returns current trip mileage
    fn get_current_mileage(&self) -> u32;
}

pub trait TotalMileage {
    // Returns total overall mileage
    fn get_total_mileage(&self) -> u32;
}

pub trait Longitude {
    // Returns current longitude
    fn get_longitude(&self) -> DMM;
}

pub trait Latitude {
    // Returns current latitude
    fn get_latitude(&self) -> DMM;
}

pub trait Date {
    // Returns current date
    fn get_date(&self) -> UnitDate;
}

pub trait Time {
    // Returns current time
    fn get_time(&self) -> UnitTime;
}