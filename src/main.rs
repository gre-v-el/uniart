mod luminance_printer;
mod colors;
mod pixels_printer;
mod braille_printer;
mod edges_printer;
mod shape_printer;
mod image_file;
mod gif_animator;

use std::process::exit;

use clap::Parser;
use gif_animator::animate_gif;
use image::{DynamicImage, ImageError};

use image_file::ImageFile;

const MODES: [&str; 6] = ["luminance", "pixels", "double-pixels", "braille", "edges", "shapes"];

/// CLI app to display images in the terminal in different ways
#[derive(Parser)]
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
    
    /// Sets the width of the output. If set to 0 it will fill the terminal.
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

    /// Switch the quality of output. (only works for luminance, edges and shapes modes)
    /// 
    /// For luminance and edges modes it will use a larger palette of characters,
    /// which will potentially result in a better looking output.
    /// For shapes mode it will reduce the quality of character choice and improve performance.
    #[arg(short, long, help_heading = "Image manipulation")]
    quality: bool,

    /// Uses linear filter instead of nearest neighbor when scaling the image. 
    /// 
    /// Results in cleaner but less crisp output.
    #[arg(short, long, help_heading = "Image manipulation")]
    filter: bool,


    // Not initialized by clap, filled later.
    #[clap(skip)]
    height: u32,

    #[clap(skip)]
    image_file: Option<ImageFile>,

    #[clap(skip)]
    printer: Option<fn(&Args, &DynamicImage) -> ()>
}

impl Args {
    fn normalize(&mut self) -> Result<(), ImageError> {
        if self.truecolor {
            self.colors = true;
        }

        // If the mode is not valid, try to find a mode that starts with the given string.
        if !MODES.contains(&self.mode.as_str()) {
            let mode = MODES.iter().find(|&m| m.starts_with(&self.mode));
            match mode {
                Some(m) => self.mode = m.to_string(),
                None => {
                    eprintln!("Invalid mode.");
                    exit(1);
                },
            }
        }

        // Set the printer function based on the mode.
        self.printer = Some(
            match self.mode.as_str() {
                "luminance" =>     luminance_printer::print_luminance,
                "pixels" =>        pixels_printer::print_pixels,
                "double-pixels" => pixels_printer::print_double_pixels,
                "braille" =>       braille_printer::print_braille,
                "edges" =>         edges_printer::print_edges,
                "shapes" =>        shape_printer::print_shapes,
                _ => {
                    eprintln!("Invalid mode.");
                    exit(1);
                }
            });
        if self.mode == "pixels" || self.mode == "double-pixels" {
            self.colors = true;
        }

        // Open image file.
        self.image_file = Some(ImageFile::open(&self.image)?);
        
        // Calculate the dimensions of the image in characters.
        let img_ref = self.image_file.as_ref().unwrap();
        let (w, h) = img_ref.dimensions();
        let (w, h) = (w as f32, h as f32);

        if self.width > 0 {
            self.height = (h/w*self.width as f32 / self.aspect) as u32;
        }
        else {
            let (tw, th) = termion::terminal_size().unwrap_or((100, 100));
            let (tw, th) = (tw as f32, th as f32);
            let terminal_aspect = tw / th;
            let image_aspect = w / h;

            if terminal_aspect/self.aspect > image_aspect {
                self.height = th as u32 - 2; // Leave some space for the prompt.
                self.width = (w/h*self.height as f32 * self.aspect) as u32;
            }
            else {
                self.width = tw as u32;
                self.height = (h/w*self.width as f32 / self.aspect) as u32;
            }
        }

        Ok(())
    }

    fn realize(&self) {
        match &self.image_file {
            Some(ImageFile::Image(image)) => {
                // Just print the image.
                (self.printer.unwrap())(self, image);
            },
            Some(ImageFile::Gif(frames, dims)) => {
                animate_gif(self, frames, *dims);
            },
            None => {
                eprintln!("Error opening image file.");
                exit(1);
            }
        }
    }
}

fn main() {
    let mut args = Args::parse();

    if let Err(e) = args.normalize() {
        eprintln!("{}", e);
        return;
    }
    args.realize();
}
