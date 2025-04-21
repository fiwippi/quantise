mod v1;
mod v2;

use image::{DynamicImage, Luma, Rgba};

pub trait Palette {
    // As per LMQ, these are the steps:
    //   1. Calculate initial threshold values
    //   2. Initialise the segment map including segment averages
    //   3. Calculate the thresholds:
    //      a) Segment the pixels into thresholds based on the histogram
    //      b) Calculate the segment averages
    //      c) Recalculate the thresholds
    fn palette(img: &DynamicImage, m: usize) -> Vec<Luma<u8>>;
}

pub fn greyscale(pixel: Rgba<u8>) -> u8 {
    let r = pixel.0[0] as f32;
    let g = pixel.0[1] as f32;
    let b = pixel.0[2] as f32;
    let lum = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
    lum
}

pub use v1::V1;
pub use v2::V2;
