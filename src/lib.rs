use image::{DynamicImage, GenericImageView, GrayImage};
use palette::{Palette, greyscale};

pub mod palette;

pub fn quantise<P: Palette>(img: &DynamicImage, m: usize) -> GrayImage {
    let p = P::palette(img, m);
    let mut q = GrayImage::new(img.width(), img.height());
    for x in 0..q.width() {
        for y in 0..q.height() {
            let lum = greyscale(img.get_pixel(x, y));
            let lum = *p.iter().min_by_key(|&n| u8::abs_diff(n.0[0], lum)).unwrap();
            q.put_pixel(x, y, lum);
        }
    }
    q
}
