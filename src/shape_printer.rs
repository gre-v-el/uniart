use image::GenericImageView;

use crate::{colors::set_color, Args};

// const SYMBOLS_FILE: &[u8] = include_bytes!("../assets/symbols_small.png");
// const SYMBOLS_WIDTH: u32 = 4;
// const SYMBOLS_HEIGHT: u32 = 8;

const SYMBOLS_FILE: &[u8] = include_bytes!("../assets/symbols_big.png");
const SYMBOLS_WIDTH: u32 = 8;
const SYMBOLS_HEIGHT: u32 = 16;

const ASCII_START: u32 = 32;
const ASCII_END: u32 = 126;

pub fn print_shapes(args: &Args) {
    let symbols = image::load_from_memory(SYMBOLS_FILE).unwrap();
    let symbols = symbols.as_luma8().unwrap();
    
    let colors = args.image_file.as_ref().unwrap().resize_exact(
        args.width,
        args.height, 
        if args.filter {image::imageops::FilterType::Triangle} else {image::imageops::FilterType::Nearest}
    );
    let image = args.image_file.as_ref().unwrap().resize_exact(
        args.width * SYMBOLS_WIDTH,
        args.height * SYMBOLS_HEIGHT, 
        if args.filter {image::imageops::FilterType::Triangle} else {image::imageops::FilterType::Nearest}
    );
    let image = image.to_luma8();

    for y in 0..args.height {
        for x in 0..args.width {
            let x0 = x * SYMBOLS_WIDTH;
            let y0 = y * SYMBOLS_HEIGHT;

            let mut best_symbol = 0;
            let mut best_difference = u32::MAX;

            for symbol in 0..=(ASCII_END - ASCII_START) {
                let mut symbol_difference = 0;
                for v in 0..SYMBOLS_HEIGHT {
                    for u in 0..SYMBOLS_WIDTH {
                        let symbol_luminance = symbols.get_pixel(symbol*SYMBOLS_WIDTH+u, v).0[0];
                        let mut image_luminance = image.get_pixel(x0 + u, y0 + v).0[0];
                        if args.invert { image_luminance = 255 - image_luminance; }
                        let pixel_difference = (symbol_luminance as i32 - image_luminance as i32).abs() as u32;
                        symbol_difference += pixel_difference;
                    }
                }
                if symbol_difference < best_difference {
                    best_difference = symbol_difference;
                    best_symbol = symbol;
                }
            }

            let symbol = std::char::from_u32(ASCII_START + best_symbol).unwrap();

            set_color(colors.get_pixel(x, y), args);
            print!("{}", symbol);
        }
        println!();
    }
}