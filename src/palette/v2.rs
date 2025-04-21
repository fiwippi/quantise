use std::ops::{Deref, DerefMut};

use image::{DynamicImage, GenericImageView, Luma};

use super::{Palette, greyscale};

#[derive(Clone, Copy)]
struct Histogram([u64; 255]);

impl Histogram {
    fn mean(&self) -> u64 {
        let mut sum = 0;
        let mut total = 0;
        for (i, v) in self.0.iter().enumerate() {
            sum += ((i + 1) as u64) * *v;
            total += *v;
        }
        if total == 0 {
            return 0;
        }

        sum / total
    }
}

impl Deref for Histogram {
    type Target = [u64; 255];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Histogram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Histogram {
    fn default() -> Self {
        Self([0; 255])
    }
}

impl From<&DynamicImage> for Histogram {
    fn from(img: &DynamicImage) -> Self {
        let mut hist = Self::default();

        for x in 0..img.width() {
            for y in 0..img.height() {
                let lum = greyscale(img.get_pixel(x, y));
                hist[lum as usize] += 1;
            }
        }

        hist
    }
}

pub struct V2 {}

impl Palette for V2 {
    fn palette(img: &DynamicImage, m: usize) -> Vec<Luma<u8>> {
        // Step 1
        let mut t = vec![0; m + 1];
        for i in 0..=m {
            t[i] = (i * 256) / m;
        }

        // Step 2
        let mut segments = vec![Histogram::default(); m + 1];
        let mut averages = vec![0; m + 1];

        // Step 3
        let histogram = Histogram::from(img);
        loop {
            let old_t = t.clone();

            // A
            for (k, v) in histogram.iter().enumerate() {
                if k == 0 {
                    segments[1][0] = *v;
                }
                for i in 1..=m {
                    if t[i - 1] < k && k <= t[i] {
                        segments[i][k] = *v;
                    }
                }
            }

            // B
            for i in 1..=m {
                averages[i] = segments[i].mean();
            }

            // C
            for i in 1..m {
                let a = averages[i];
                let b = averages[i + 1];
                t[i] = ((a + b) / 2) as usize;
            }

            if t == old_t {
                break;
            }
        }

        let mut colours = Vec::new();
        for i in 1..=m {
            let l = averages[i] as u8;
            colours.push(Luma([l]));
        }
        colours
    }
}
