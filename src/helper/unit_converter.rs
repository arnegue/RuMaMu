pub fn meter_to_feet(meter: f32) -> f32 {
    meter * 3.28084
}

pub fn feet_to_meter(feet: f32) -> f32 {
    feet / 3.28084
}

pub fn meter_to_fathom(meter: f32) -> f32 {
    meter * 0.54680665
}

pub fn fathom_to_meter(fathom: f32) -> f32 {
    fathom / 0.54680665
}

pub fn meter_to_nm(meter: f32) -> f32 {
    meter / 1852.0
}

pub fn nm_to_meter(nm: f32) -> f32 {
    nm * 1852.0
}

pub fn sm_to_nm(sm: f32) -> f32 {
    sm * 0.868976
}

pub fn nm_to_sm(sm: f32) -> f32 {
    sm / 0.868976
}

pub fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 1.8 + 32.0
}

pub fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}
