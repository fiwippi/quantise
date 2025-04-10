use std::collections::HashMap;

use clap::Parser;
use image::{DynamicImage, GenericImageView, GrayImage, ImageReader, Luma, Rgba};

/// Program to quantise an image
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input filepath
    #[arg(short, long, required = true)]
    input: String,

    /// Output filepath
    #[arg(short, long, required = true)]
    output: String,

    /// How many discrete greyscale hues to quantise into
    #[arg(short, long, required = true)]
    colours: usize,
}

fn greyscale(pixel: Rgba<u8>) -> u8 {
    let r = pixel.0[0] as f32;
    let g = pixel.0[1] as f32;
    let b = pixel.0[2] as f32;
    let lum = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
    lum
}

struct Histogram(HashMap<u8, usize>);

impl Histogram {
    fn mean(&self) -> usize {
        if self.0.is_empty() {
            return 0;
        }

        let mut sum = 0;
        let mut total = 0;
        for (k, v) in self.0.iter() {
            sum += (*k as usize) * *v;
            total += *v;
        }

        sum / total
    }
}

impl Default for Histogram {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl From<&DynamicImage> for Histogram {
    fn from(img: &DynamicImage) -> Self {
        let mut hist = Self(HashMap::new());

        for x in 0..img.width() {
            for y in 0..img.height() {
                let lum = greyscale(img.get_pixel(x, y));
                *hist.0.entry(lum).or_insert(0) += 1;
            }
        }

        hist
    }
}

fn palette(img: &DynamicImage, h: usize) -> Vec<u8> {
    let histogram = Histogram::from(img);

    // Calculate the initial threshold values
    let mut t = vec![0; h + 1];
    for i in 0..=h {
        t[i] = (i * 256) / h;
    }
    // Initialising the segment map
    let mut segments: HashMap<usize, Histogram> = HashMap::new();
    for i in 0..=h {
        segments.insert(i, Histogram::default());
    }
    // Initialising the averages for each segment
    let mut averages: HashMap<usize, usize> = HashMap::new();

    // Calculate the thresholds
    loop {
        let old_t = t.clone();

        // Segments the pixels of the image into thresholds based on histogram
        for (k, v) in histogram.0.iter() {
            // Checks for k=0 since it cannot be checked in the loop
            if *k == 0 {
                segments.entry(1).or_default().0.insert(0, *v);
            }
            // Checking in general
            for i in 1..=h {
                if t[i - 1] < *k as usize && *k as usize <= t[i] {
                    segments.entry(i).or_default().0.insert(*k, *v);
                }
            }
        }

        // Calculating the segment averages
        for i in 1..=h {
            averages.insert(i, segments.entry(i).or_default().mean());
        }
        // Recalculating the thresholds
        for i in 1..h {
            let a = averages.get(&i).unwrap();
            let b = averages.get(&(i + 1)).unwrap();
            t[i] = (a + b) / 2
        }
        // If the old threshold is equal to the new threshold we are done
        if t == old_t {
            break;
        }
    }

    let mut colours = Vec::new();
    for i in 1..=h {
        let l = *averages.get(&i).unwrap() as u8;
        colours.push(l);
    }
    colours
}

fn quantise(img: &DynamicImage, h: usize) -> GrayImage {
    let p = palette(img, h);

    let mut q = GrayImage::new(img.width(), img.height());
    for x in 0..q.width() {
        for y in 0..q.height() {
            let lum = greyscale(img.get_pixel(x, y));
            let lum = *p.iter().min_by_key(|&n| u8::abs_diff(*n, lum)).unwrap();
            q.put_pixel(x, y, Luma([lum]));
        }
    }

    q
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let input = ImageReader::open(args.input)?.decode()?;
    quantise(&input, args.colours).save(args.output)?;

    Ok(())
}
