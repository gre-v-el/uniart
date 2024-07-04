mod luminance_printer;
mod colored_printer;

use clap::Parser;
use image::ImageError;

/// CLI app to process images with various modes
#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Gabriel Myszkier <myszkier.gabriel@gmail.com>", about = "Converts images to ascii art")]
struct Args {
    /// Path to the image file
    image: String,

    /// Sets the width of the output
    #[arg(short, long, default_value_t = 100)]
    width: u32,

    /// Outputs the image in color (ansi color escape sequences)
    #[arg(short, long)]
    colors: bool,

    /// Inverts the image brightness (useful in white-background terminals)
    #[arg(short, long)]
    invert: bool,

    /// Luminance mode [default]
    #[arg(short, long)]
    luminance: bool,

    /// Pixels mode
    #[arg(short, long)]
    pixels: bool,

    /// Edge detection mode
    #[arg(short, long)]
    edges: bool,

    /// Shapes detection mode
    #[arg(short, long)]
    shapes: bool,

    /// Sets the aspect ratio of the terminal font
    #[arg(short, long, default_value_t = 2.0)]
    font_aspect_ratio: f32,
}

impl Args {
    fn validate(&mut self) {
        let modes = 
            if self.luminance {1} else {0} + 
            if self.edges {1} else {0} + 
            if self.pixels {1} else {0} + 
            if self.shapes {1} else {0};
        if modes > 1 {
            eprintln!("Only one mode can be selected at a time");
            std::process::exit(1);
        }
        if modes == 0 {
            self.luminance = true;
        }
    }

    fn realize(&self) -> Result<(), ImageError> {
        let image = image::open(&self.image)?;

        let (w, h) = (image.width() as f32, image.height() as f32);
        let scaled = image.resize_exact(
            self.width,
            (h/w*self.width as f32 / self.font_aspect_ratio) as u32, 
            image::imageops::FilterType::Nearest
        );


        if self.luminance {
            luminance_printer::print_luminance(&self, &scaled)?;
        } else if self.edges {
            todo!()
        } else if self.pixels {
            todo!()
        }else {
            todo!()
        }

        Ok(())
    }
}

fn main() {
    let mut args = Args::parse();
    args.validate();
    match args.realize() {
        Ok(_) => {},
        Err(e) => eprintln!("{}", e)
    }
}
