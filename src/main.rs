mod luminance_converter;

use clap::Parser;
use image::{DynamicImage, ImageError};

/// CLI app to process images with various modes
#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Gabriel Myszkier <myszkier.gabriel@gmail.com>", about = "Converts images to ascii art")]
struct Args {
    /// Path to the image file
    image: String,

    /// Sets the width of the output
    #[arg(short, long, default_value_t = 100)]
    width: u32,

    /// Outputs the image in color
    #[arg(short, long)]
    colors: bool,

    /// Inverts the image brightness (useful in white-background terminals)
    #[arg(short, long)]
    invert: bool,

    /// Luminance mode [default]
    #[arg(short, long)]
    luminance: bool,

    /// Edge detection mode
    #[arg(short, long)]
    edge: bool,

    /// Shapes detection mode
    #[arg(short, long)]
    shapes: bool,

    /// Sets the aspect ratio of the terminal font
    #[arg(short, long, default_value_t = 2.0)]
    font_aspect_ratio: f32,
}

impl Args {
    fn validate(&mut self) {
        let modes = if self.luminance {1} else {0} + if self.edge {1} else {0} + if self.shapes {1} else {0};
        if modes > 1 {
            eprintln!("Only one mode can be selected at a time");
            std::process::exit(1);
        }
        if modes == 0 {
            self.luminance = true;
        }
    }

    fn realize(&self) -> Result<(Vec<String>, DynamicImage), ImageError>{
        let image = image::open(&self.image)?;

        let (w, h) = (image.width() as f32, image.height() as f32);
        let scaled = image.resize_exact(
            self.width,
            (h/w*self.width as f32 / self.font_aspect_ratio) as u32, 
            image::imageops::FilterType::Nearest
        );


        let result = 
            if self.luminance {
                luminance_converter::convert_luminance(&scaled)
            } else if self.edge {
                todo!()
            } else {
                todo!()
            };

        return match result {
            Ok(v) => Ok((v, scaled)),
            Err(e) => Err(e),
        }
    }
}

fn print_with_colors(lines: &Vec<String>, image: &DynamicImage) {
    todo!()
}

fn main() {
    let mut args = Args::parse();
    args.validate();
    match args.realize() {
        Ok((lines, image)) => {
            if args.colors {
                print_with_colors(&lines, &image);
            }
            else {
                for line in lines {
                    println!("{line}");
                }
            }
        },
        Err(e) => panic!("{}", e.to_string()),
    };
}
