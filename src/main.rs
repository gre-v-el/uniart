mod luminance_printer;
mod colored_printer;
mod pixels_printer;
mod braille_printer;
mod edges_printer;

use clap::Parser;
use image::{DynamicImage, ImageError};

/// CLI app to display images in the terminal in different ways
#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Gabriel Myszkier <myszkier.gabriel@gmail.com>", about = "Converts images to ascii art")]
struct Args {
    /// Mode (luminance, pixels, double-pixels, braille)
    #[arg(short, long, default_value_t = String::from("luminance"))]
    mode: String,

    /// Path to the image file
    image: String,

    /// Sets the width of the output. If set to 0 it will fill the terminal width.
    #[arg(short, long, default_value_t = 100)]
    width: u32,

    /// Outputs the image in color (ansi escape sequences)
    #[arg(short, long)]
    colors: bool,

    /// Use truecolor escape sequences (only works in some terminals)
    #[arg(short, long)]
    truecolor: bool,

    /// Inverts the image brightness (useful in white-background terminals)
    #[arg(short, long)]
    invert: bool,

    /// Dense palette / more characters (only works for luminance mode) 
    #[arg(short, long)]
    dense: bool,

    /// Use linear filtering instead of nearest neighbor when scaling the image
    #[arg(short, long)]
    filter: bool,

    /// Sets the aspect ratio of the terminal font
    #[arg(short, long, default_value_t = 2.0)]
    aspect_ratio: f32,


    #[clap(skip)]
    height: u32,
    #[clap(skip)]
    image_file: Option<DynamicImage>,
}

impl Args {
    fn validate(&mut self) -> Result<(), ImageError> {
        if self.width == 0 {
            self.width = match term_size::dimensions() {
                Some((w, _)) => w as u32,
                None => {
                    eprintln!("Could not determine terminal width. Setting width to 100");
                    100
                },
            }
        }

        self.image_file = Some(image::open(&self.image)?);
        let (w, h) = (self.image_file.as_ref().unwrap().width() as f32, self.image_file.as_ref().unwrap().height() as f32);
        self.height = (h/w*self.width as f32 / self.aspect_ratio) as u32;

        if self.truecolor {
            self.colors = true;
        }

        Ok(())
    }

    fn realize(&self) {
        if self.mode == "luminance"{
            luminance_printer::print_luminance(&self);
        } 
        else if self.mode == "pixels" {
            pixels_printer::print_pixels(&self);
        } 
        else if self.mode == "double-pixels"{
            pixels_printer::print_double_pixels(&self);
        } 
        else if self.mode == "braille" {
            braille_printer::print_braille(&self);
        }
        else if self.mode == "edges" {
            edges_printer::print_edges(&self);
        }
        else {
            eprintln!("Invalid mode.")
        }
    }
}

fn main() {
    let mut args = Args::parse();

    if let Err(e) = args.validate() {
        eprintln!("{}", e);
    }
    args.realize();
}
