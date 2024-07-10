use std::f32::consts::PI;

use image::{DynamicImage, GenericImageView, ImageBuffer};

use crate::{colors::{reset_color, set_black_background, set_color_full_brightness}, luminance_printer::char_from_color, Args};

const SOBEL_X: [i16; 9] = [
    -2, 0, 2,
    -3, 0, 3,
    -2, 0, 2
];

const SOBEL_Y: [i16; 9] = [
    -2, -3, -2,
     0,  0,  0,
     2,  3,  2
];

const EDGES: [char; 4] = ['|', '/', '-', '\\'];

pub fn convolve(image: &DynamicImage, kernel: &[i16; 9]) -> ImageBuffer<image::Luma<i16>, Vec<i16>> {
    ImageBuffer::from_fn(image.width(), image.height(), |x, y| {
        let mut sum = 0;
        for i in 0..3usize {
            for j in 0..3usize {
                let mut x0 = x as i32 + i as i32;
                let mut y0 = y as i32 + j as i32;
                // If the pixel is out of bounds, snap it to the nearest edge.
                if x0 < 0 || x0 >= image.width() as i32 {
                    x0 = x as i32;
                }
                if y0 < 0 || y0 >= image.height() as i32 {
                    y0 = y as i32;
                }
                
                // Sum the changes in all channels.
                let p = image.get_pixel(x0 as u32, y0 as u32).0;
                for c in p {
                    sum += c as i16 * kernel[i*3 + j];
                }
            }
        }
        image::Luma([sum])
    })
}

pub fn print_edges(args: &Args) {
    let image = args.image_file.as_ref().unwrap();
    let image = image.resize_exact(
        args.width, 
        args.height, 
        if args.filter {image::imageops::FilterType::Triangle} else {image::imageops::FilterType::Nearest}
    );
    
    let edges_x =  convolve(&image, &SOBEL_X);
    let edges_y =  convolve(&image, &SOBEL_Y);

    for y in 0..image.height() {
        set_black_background(args);
        for x in 0..image.width() {
            let p_x = edges_y.get_pixel(x, y).0[0] as f32;
            let p_y = edges_x.get_pixel(x, y).0[0] as f32;
            let p = (p_x*p_x + p_y*p_y).sqrt();

            let pixel  = image.get_pixel(x, y);
            set_color_full_brightness(pixel, args);
            
            if p < 1000.0 { 
                // print luma
                print!("{}", char_from_color(pixel, args));
            }
            else { 
                // print appropriate edge character
                let mut angle = f32::atan2(p_y, p_x);
                if angle < 0.0 { angle += PI; }
                let i = ((angle + PI/8.0) / (PI/4.0)) as usize % 4;
                print!("{}", EDGES[i]);
            }
        }
        reset_color();
        println!();
    }
}