use image::{DynamicImage, GenericImageView};

use crate::{colors::{reset_color, set_black_background, set_color}, Args};

pub const SMALL_SYMBOLS_FILE: &[u8] = include_bytes!("../assets/symbols_small.png");
const SMALL_SYMBOLS_WIDTH: u32 = 4;
const SMALL_SYMBOLS_HEIGHT: u32 = 8;

pub const BIG_SYMBOLS_FILE: &[u8] = include_bytes!("../assets/symbols_big.png");
const BIG_SYMBOLS_WIDTH: u32 = 8;
const BIG_SYMBOLS_HEIGHT: u32 = 16;

pub const ASCII_START: u32 = 32;
pub const ASCII_END: u32 = 127;

pub fn print_shapes(args: &Args, image: &DynamicImage) {
    let symbols_width =  if args.quality {SMALL_SYMBOLS_WIDTH}  else {BIG_SYMBOLS_WIDTH};
    let symbols_height = if args.quality {SMALL_SYMBOLS_HEIGHT} else {BIG_SYMBOLS_HEIGHT};

    let symbols = args.shapes_symbols.as_ref().unwrap();
    
    // One pixel per character.
    let colors = image.resize_exact(
        args.width,
        args.height, 
        if args.filter {image::imageops::FilterType::Triangle} else {image::imageops::FilterType::Nearest}
    );

    // WxH pixels per character, according to the selected quality.
    let image = image.resize_exact(
        args.width * symbols_width,
        args.height * symbols_height, 
        if args.filter {image::imageops::FilterType::Triangle} else {image::imageops::FilterType::Nearest}
    );
    let image = image.to_luma8();

    for y in 0..args.height {
        set_black_background(args);
        for x in 0..args.width {
            let x0 = x * symbols_width;
            let y0 = y * symbols_height;

            // select the best symbol for the current area
            let mut best_symbol = 0;
            let mut best_cost = u32::MAX;

            for c in args.palette.as_ref().unwrap().chars() {
                let symbol = c as u32 - ASCII_START;
                let mut symbol_cost = 0;
                for v in 0..symbols_height {
                    for u in 0..symbols_width {
                        let symbol_luminance = symbols.get_pixel(symbol*symbols_width+u, v).0[0];
                        let mut image_luminance = image.get_pixel(x0 + u, y0 + v).0[0];
                        if args.invert { image_luminance = 255 - image_luminance; }
                        let pixel_difference = (symbol_luminance as i32 - image_luminance as i32).abs() as u32;
                        symbol_cost += pixel_difference;
                    }
                }
                if symbol_cost < best_cost {
                    best_cost = symbol_cost;
                    best_symbol = symbol;
                }
            }

            let symbol = std::char::from_u32(ASCII_START + best_symbol).unwrap();

            set_color(colors.get_pixel(x, y), args);
            print!("{}", symbol);
        }
        reset_color();
        println!();
    }
}