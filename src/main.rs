mod luminance_converter;

use clap::Parser;

/// CLI app to process images with various modes
#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Your Name <your.email@example.com>", about = "Converts images to ascii art")]
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

fn main() {
    let mut args = Args::parse();

    let modes = if args.luminance {1} else {0} + if args.edge {1} else {0} + if args.shapes {1} else {0};
    if modes > 1 {
        eprintln!("Only one mode can be selected at a time");
        std::process::exit(1);
    }
    if modes == 0 {
        args.luminance = true;
    }
    
    let result = 
        if args.luminance {
            luminance_converter::convert_luminance(&args)
        } else if args.edge {
            todo!()
        } else {
            todo!()
        };
}
