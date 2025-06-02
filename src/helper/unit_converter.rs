pub fn meter_to_feet(meter: f64) -> f64 {
    meter * 3.28084
}

pub fn feet_to_meter(feet: f64) -> f64 {
    feet / 3.28084
}

pub fn meter_to_fathom(meter: f64) -> f64 {
    meter * 0.54680665
}

pub fn fathom_to_meter(fathom: f64) -> f64 {
    fathom / 0.54680665
}

pub fn meter_to_nm(meter: f64) -> f64 {
    meter / 1852.0
}

pub fn nm_to_meter(nm: f64) -> f64 {
    nm * 1852.0
}

pub fn sm_to_nm(sm: f64) -> f64 {
    sm * 0.868976
}

pub fn nm_to_sm(sm: f64) -> f64 {
    sm / 0.868976
}

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}
