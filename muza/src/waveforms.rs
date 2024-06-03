use crate::aliases::{Part, Sample};
use std::f64::consts::PI;

pub type WaveForm = fn(Part) -> Sample;

pub fn sin(part: Part) -> Sample {
    (2. * PI * part).sin()
}

pub fn tri(part: Part) -> Sample {
    if part < 0.25 {
        return 4. * part;
    }
    if part < 0.75 {
        return 2. - 4. * part;
    }
    4. * part - 4.
}

pub fn sqr(part: Part) -> Sample {
    if part < 0.5 {
        return 1.;
    }
    -1.
}

pub fn saw(part: Part) -> Sample {
    1. - 2. * part
}
