// Some user defined Units

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8
}

// Degrees, Minutes, Seconds (DMS)
#[derive(Debug, Copy, Clone)]
pub struct DMS {
    pub degrees: u8, // Latitude max 90, Longitude max 180
    pub minutes: u8, // Always between 0-59
    pub seconds: f64,
    pub direction: char, // 'N', 'S', 'E', or 'W'
}

// Degrees Decimal Minutes (DMM)
#[derive(Debug, Copy, Clone)]
pub struct DMM {
    pub degrees: u8,     // Latitude max 90, Longitude max 180
    pub minutes: f64,    // Always between roughly 0.0-59.999
    pub direction: char, // 'N', 'S', 'E', or 'W'
}
