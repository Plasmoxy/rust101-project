use image::{ImageBuffer, Pixel, Rgb, RgbImage};
use rand::Rng;
use std::{cmp::min, thread};

pub struct Processing {}

impl Processing {
    // Basic negative directly through enumeration
    pub fn negative_basic(buf: &mut RgbImage) {
        for (_x, _y, px) in buf.enumerate_pixels_mut() {
            // pixel.invert();
            // let slice = &mut px.0; // we can use directly the rgba enum
            px[0] = 255 - px[0];
            px[1] = 255 - px[1];
            px[2] = 255 - px[2];
            // keep alpha
        }
    }

    pub fn wobble(buf: &mut RgbImage) {
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

    // `crop_image` takes an image and the dimensions of the desired crop and returns a new image that is the cropped portion of the original image
    // x, y -> coordinates of the upper left edge of desired cropped rectangle. Width/height represent the width/height of this rectangle.
    pub fn crop_image(img: &RgbImage, x: u32, y: u32, width: u32, height: u32) -> RgbImage {
        // Determine the x-coordinate of the right edge of the crop area
        let x_end = min(x + width, img.width());
        // Determine the y-coordinate of the bottom edge of the crop area
        let y_end = min(y + height, img.height());
        // Create a new image buffer to hold the cropped image
        let mut cropped_img = ImageBuffer::new(x_end - x, y_end - y);

        // Iterate over the pixels in the cropped image buffer and copy the corresponding pixels from the original image
        for (x_cropped, y_cropped, pixel) in cropped_img.enumerate_pixels_mut() {
            // Determine the corresponding pixel coordinates in the original image
            let x_original = x + x_cropped;
            let y_original = y + y_cropped;

            // If the original pixel is within the crop area, copy its value to the cropped image
            if x_original < x_end && y_original < y_end {
                *pixel = img.get_pixel(x_original, y_original).to_rgb();
            }
        }

        cropped_img
    }

    // angle is in degrees
    pub fn rotate(img: &RgbImage, angle: f32) -> RgbImage {
        let (orig_width, orig_height) = img.dimensions();
        let mut rotated_width = orig_width;
        let mut rotated_height = orig_height;

        // in case the image is rotated on 90 or 270 degrees,
        // the width and height will be swaped to remove black borders in the output image
        if ((angle / 90.0) % 2.0) == 1.0 {
            rotated_width = orig_height;
            rotated_height = orig_width;
        }

        let mut rotated = ImageBuffer::new(rotated_width, rotated_height);

        let sin_a = angle.to_radians().sin();
        let cos_a = angle.to_radians().cos();

        let width_center = orig_width as f32 / 2.0;
        let height_center = orig_height as f32 / 2.0;

        for (x, y, pixel) in img.enumerate_pixels() {
            // Compute the new position of the pixel in the rotated image
            // (rotation formulas were taken from https://homepages.inf.ed.ac.uk/rbf/HIPR2/rotate.htm)
            let new_x = (cos_a * (x as f32 - width_center) - sin_a * (y as f32 - height_center) + rotated_width as f32 / 2.0) as u32;
            let new_y = (sin_a * (x as f32 - width_center) + cos_a * (y as f32 - height_center) + rotated_height as f32 / 2.0) as u32;

            if new_x < rotated_width && new_y < rotated_height {
                // Copy the pixel from the original image to the rotated image
                rotated.put_pixel(new_x, new_y, pixel.to_rgb());
            }
        }

        rotated
    }

    fn is_a_shade_of_black(pixel: &Rgb<u8>, threshold: u8) -> bool {
        pixel[0] <= threshold && pixel[1] <= threshold && pixel[2] <= threshold
    }

    fn find_top_edge(image: &RgbImage, threshold: u8) -> u32 {
        let (width, height) = image.dimensions();
        let mut top = height - 1;
        for x in 0..width {
            for y in 0..height {
                if !Self::is_a_shade_of_black(image.get_pixel(x, y), threshold) && y < top {
                    top = y;
                    break;
                }
                if y == top {
                    break;
                }
            }
        }

        top
    }

    fn find_left_edge(image: &RgbImage, threshold: u8) -> u32 {
        let (width, height) = image.dimensions();
        let mut left = width - 1;
        for y in 0..height {
            for x in 0..width {
                if !Self::is_a_shade_of_black(image.get_pixel(x, y), threshold) && x < left {
                    left = x;
                    break;
                }
                if x == left {
                    break;
                }
            }
        }

        left
    }

    fn find_bottom_edge(image: &RgbImage, threshold: u8) -> u32 {
        let (width, height) = image.dimensions();
        let mut bottom = 0;
        for x in 0..width {
            for y in (0..height).rev() {
                if !Self::is_a_shade_of_black(image.get_pixel(x, y), threshold) && y > bottom {
                    bottom = y;
                    break;
                }
                if y == bottom {
                    break;
                }
            }
        }

        bottom
    }

    fn find_right_edge(image: &RgbImage, threshold: u8) -> u32 {
        let (width, height) = image.dimensions();
        let mut right = 0;
        for y in 0..height {
            for x in (0..width).rev() {
                if !Self::is_a_shade_of_black(image.get_pixel(x, y), threshold) && x > right {
                    right = x;
                    break;
                }
                if x == right {
                    break;
                }
            }
        }

        right
    }

    pub fn remove_borders(image: &RgbImage) -> Result<RgbImage, &'static str> {
        let threshold = 40;

        // Find top edge
        let image2 = image.clone();
        let top_edge_handle = thread::spawn(move || Self::find_top_edge(&image2, threshold));

        // Find left edge
        let image3 = image.clone();
        let left_edge_handle = thread::spawn(move || Self::find_left_edge(&image3, threshold));

        // Find bottom edge
        let image4 = image.clone();
        let bottom_edge_handle = thread::spawn(move || Self::find_bottom_edge(&image4, threshold));

        // Find right edge
        let image5 = image.clone();
        let right_edge_handle = thread::spawn(move || Self::find_right_edge(&image5, threshold));

        let top = top_edge_handle.join().unwrap();
        let left = left_edge_handle.join().unwrap();
        let bottom = bottom_edge_handle.join().unwrap();
        let right = right_edge_handle.join().unwrap();

        println!("LOG: {} {} {} {}", left, top, bottom, right);

        if right < left {
            return Err("provided image is completely black");
        }

        Ok(Self::crop_image(image, left, top, right - left + 1, bottom - top + 1))
    }
}
