mod luminance_printer;
mod colors;
mod pixels_printer;
mod braille_printer;
mod edges_printer;
mod shape_printer;

use clap::Parser;
use image::{DynamicImage, ImageError};

const MODES: [&str; 6] = ["luminance", "pixels", "double-pixels", "braille", "edges", "shapes"];

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
    image_file: Option<DynamicImage>,
}

impl Args {
    fn validate(&mut self) -> Result<(), ImageError> {
        // If the width is set to 0, fill the terminal width.
        if self.width == 0 {
            self.width = match term_size::dimensions() {
                Some((w, _)) => w as u32,
                None => {100},
            }
        }

        // Open image file and calculate height in characters.
        self.image_file = Some(image::open(&self.image)?);
        let img_ref = self.image_file.as_ref().unwrap();
        let (w, h) = (img_ref.width() as f32, img_ref.height() as f32);
        self.height = (h/w*self.width as f32 / self.aspect) as u32;

        if self.truecolor {
            self.colors = true;
        }

        // If the mode is not valid, try to find a mode that starts with the given string.
        if !MODES.contains(&self.mode.as_str()) {
            let mode = MODES.iter().find(|&m| m.starts_with(&self.mode));
            match mode {
                Some(m) => self.mode = m.to_string(),
                None => {},
            }
        }

        Ok(())
    }

    fn realize(&self) {
        // Choose the correct printer based on the mode.
        match self.mode.as_str() {
            "luminance" =>     luminance_printer::print_luminance(&self),
            "pixels" =>        pixels_printer::print_pixels(&self),
            "double-pixels" => pixels_printer::print_double_pixels(&self),
            "braille" =>       braille_printer::print_braille(&self),
            "edges" =>         edges_printer::print_edges(&self),
            "shapes" =>        shape_printer::print_shapes(&self),
            _ => eprintln!("Invalid mode.")
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
