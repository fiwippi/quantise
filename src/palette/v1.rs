use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use image::{DynamicImage, GenericImageView, Luma};

use super::{Palette, greyscale};

struct Histogram(HashMap<u8, usize>);

impl Histogram {
    fn mean(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        let mut sum = 0;
        let mut total = 0;
        for (k, v) in self.iter() {
            sum += (*k as usize) * *v;
            total += *v;
        }
        if total == 0 {
            return 0;
        }

        sum / total
    }
}

impl Deref for Histogram {
    type Target = HashMap<u8, usize>;

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
        Self(HashMap::new())
    }
}

impl From<&DynamicImage> for Histogram {
    fn from(img: &DynamicImage) -> Self {
        let mut hist = Self::default();
        for x in 0..img.width() {
            for y in 0..img.height() {
                let lum = greyscale(img.get_pixel(x, y));
                let e = hist.entry(lum).or_insert(0);
                *e += 1;
            }
        }
        hist
    }
}

pub struct V1 {}

impl Palette for V1 {
    fn palette(img: &DynamicImage, m: usize) -> Vec<Luma<u8>> {
        // Step 1
        let mut t = vec![0; m + 1];
        for i in 0..=m {
            t[i] = (i * 256) / m;
        }

        // Step 2
        let mut segments: HashMap<usize, Histogram> = HashMap::new();
        for i in 0..=m {
            segments.insert(i, Histogram::default());
        }
        let mut averages: HashMap<usize, usize> = HashMap::new();

        // Step 3
        let histogram = Histogram::from(img);
        loop {
            let old_t = t.clone();

            // A
            for (k, v) in histogram.iter() {
                if *k == 0 {
                    segments.entry(1).or_default().insert(0, *v);
                }
                for i in 1..=m {
                    if t[i - 1] < *k as usize && *k as usize <= t[i] {
                        segments.entry(i).or_default().insert(*k, *v);
                    }
                }
            }

            // B
            for i in 1..=m {
                averages.insert(i, segments.entry(i).or_default().mean());
            }

            // C
            for i in 1..m {
                let a = averages.get(&i).unwrap();
                let b = averages.get(&(i + 1)).unwrap();
                t[i] = (a + b) / 2
            }

            if t == old_t {
                break;
            }
        }

        let mut colours = Vec::new();
        for i in 1..=m {
            let l = *averages.get(&i).unwrap() as u8;
            colours.push(Luma([l]));
        }
        colours
    }
}
