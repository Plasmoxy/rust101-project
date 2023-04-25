use std::ops::Deref;

use image::{Pixel, Rgba, RgbaImage};
use rand::Rng;
use rayon::prelude::*;

// TODO: DOESNT WORK, NO PARALEL ITERATOR FOR PIXELSKS? ???? -> can we implement par iter according to image lib?
fn inverto_bad<'a, P: Pixel + 'a>(pixels: impl rayon::iter::ParallelIterator<Item = (u32, u32, &'a mut P)>)
where
    P::Subpixel: 'a,
{
    pixels.for_each(|(x, y, px)| {
        px.invert();
    })
}

// Basic negative directly through enumeration
pub fn negative_basic(buf: &mut RgbaImage) {
    for (x, y, px) in buf.enumerate_pixels_mut() {
        // pixel.invert();
        // let slice = &mut px.0; // we can use directly the rgba enum
        px[0] = 255 - px[0];
        px[1] = 255 - px[1];
        px[2] = 255 - px[2];
        // keep alpha
    }
}

pub fn wobble(buf: &mut RgbaImage) {
    let diff = 100;
    let prev = buf.clone();
    let mut rng = rand::thread_rng();
    for (x, y, px) in buf.enumerate_pixels_mut() {
        let dx: i32 = diff - (rng.gen::<u32>() as i32 % (diff * 2));
        if let Some(target) = prev.get_pixel_checked((x as i32 + dx).try_into().unwrap_or(x), y) {
            *px = *target;
        }
    }
}
