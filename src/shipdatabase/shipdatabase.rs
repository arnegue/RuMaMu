type Device = u8; // TODO not u8, just a placeholder

// Currently u8 are only placeholders
type Distance = u8;
type Longitude = u8;
type Latitude = u8;
type Position = (Longitude, Latitude);
type Time = u8;
type Date = u8;
type Speed = u8;
type MagneticDegree = u16; // TODO better name? (0-359°)
type Temperature = u8;

// Instance of an datapoint
pub struct DataPoint<T> {
    data: T,                   // Actual data being held
    creation_time_point: Time, // TimePoint this instance got created
    creator: Device,           // Creator of instance (avoiding sending the same message back)
}

pub struct ShipDataBase {
    // Position/Satellite/Date
    utc_time: Option<DataPoint<Time>>,
    date: Option<DataPoint<Date>>,
    position: Option<DataPoint<Position>>,
    // target_waypoints: Option<DataPoint<Iterator<Position>>>, // TODO currently unsure how to declare this

    // Course/Heading
    course_over_ground_degree_true: Option<DataPoint<MagneticDegree>>,
    course_over_ground_degree_magnetic: Option<DataPoint<MagneticDegree>>,
    heading_degree_true: Option<DataPoint<MagneticDegree>>,
    heading_degree_magnetic: Option<DataPoint<MagneticDegree>>,

    // Speed
    speed_over_ground_knots: Option<DataPoint<Speed>>,
    speed_through_water_knots: Option<DataPoint<Speed>>,

    // Wind
    true_wind_speed_knots: Option<DataPoint<Speed>>,
    true_wind_speed_angle: Option<DataPoint<MagneticDegree>>,
    apparent_wind_speed_knots: Option<DataPoint<Speed>>,
    apparent_wind_angle: Option<DataPoint<MagneticDegree>>,

    // Mileage
    trip_mileage_miles: Option<DataPoint<Distance>>,
    total_mileage_miles: Option<DataPoint<Distance>>,

    // Water
    depth_below_transducer: Option<DataPoint<Distance>>,
    water_temperature_celsius: Option<DataPoint<Temperature>>,

    // Seatalk-Specific
    light_intensity: Option<DataPoint<u8>>,
    // List of messages which cannot be parsed, but are getting forwarded on other devices supporting these messages
    //  list_unknown_seatalk_messages: [Option<DataPoint<SeatalkMessage>] // TODO use pointers? Where are these messages held anyway?
}

impl ShipDataBase {
    fn new() -> ShipDataBase {
        ShipDataBase {
            utc_time: None,
            date: None,
            position: None,
            course_over_ground_degree_true: None,
            course_over_ground_degree_magnetic: None,
            heading_degree_true: None,
            heading_degree_magnetic: None,
            speed_over_ground_knots: None,
            speed_through_water_knots: None,
            true_wind_speed_knots: None,
            true_wind_speed_angle: None,
            apparent_wind_speed_knots: None,
            apparent_wind_angle: None,
            trip_mileage_miles: None,
            total_mileage_miles: None,
            depth_below_transducer: None,
            water_temperature_celsius: None,
            light_intensity: None,
        }
    }
}
