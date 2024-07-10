mod luminance_printer;
mod colors;
mod pixels_printer;
mod braille_printer;
mod edges_printer;
mod shape_printer;

use clap::Parser;
use image::{DynamicImage, ImageError};

/// CLI app to display images in the terminal in different ways
#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Gabriel Myszkier <myszkier.gabriel@gmail.com>", about = "Converts images to ascii art")]
#[group()]
struct Args {
    /// Path to the image file
    image: String,

    /// Mode. (one of: luminance, pixels, double-pixels, braille, edges, shapes)
    ///
    /// Can use only the first letter of the mode as a shortcut.
    #[arg(short, long, default_value_t = String::from("shapes"), help_heading = "Output customization")]
    mode: String,
    
    /// Sets the width of the output. If set to 0 it will fill the terminal width.
    /// 
    /// If the terminal width cannot be determined, the default value will be used.
    #[arg(short, long, default_value_t = 100, help_heading = "Output customization")]
    width: u32,

    /// Sets the aspect ratio of the terminal font.
    #[arg(short, long, default_value_t = 2.0, help_heading = "Output customization")]
    aspect: f32,

    /// Outputs the image in color.
    /// 
    /// Ansi escape sequences are not supported by all terminals, especially on older Windows versions.
    #[arg(short, long, help_heading = "Color options")]
    colors: bool,

    /// Uses truecolor escape sequences. (only works in some terminals)
    #[arg(short, long, help_heading = "Color options")]
    truecolor: bool,

    /// Sets the background color to black.
    #[arg(short, long, help_heading = "Color options")]
    background: bool,

    /// Inverts the image brightness. (useful in white-background terminals)
    #[arg(short, long, help_heading = "Image manipulation")]
    invert: bool,

    /// Uses a dense character palette. (only works for luminance and edges modes) 
    #[arg(short, long, help_heading = "Image manipulation")]
    dense: bool,

    /// Uses linear filter instead of nearest neighbor when scaling the image. 
    /// 
    /// Results in cleaner but less crisp output.
    #[arg(short, long, help_heading = "Image manipulation")]
    filter: bool,

    // Not initialized by clap, filled later.
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
                None => {100},
            }
        }

        self.image_file = Some(image::open(&self.image)?);
        let (w, h) = (self.image_file.as_ref().unwrap().width() as f32, self.image_file.as_ref().unwrap().height() as f32);
        self.height = (h/w*self.width as f32 / self.aspect) as u32;

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
        else if self.mode == "shapes" {
            shape_printer::print_shapes(&self);
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
        return;
    }
    args.realize();
}
