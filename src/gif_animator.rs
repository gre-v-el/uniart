use std::{io::Write, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread, time::Instant};

use image::DynamicImage;
use termion::{cursor::DetectCursorPos, raw::IntoRawMode};

use crate::{image_file::GifFrame, Args};

pub fn animate_gif(args: &Args, frames: &Vec<GifFrame>, dims: (u32, u32)) {
    // Make sure the cursor is shown when the program exits.
    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);

    ctrlc::set_handler(move || {
        running.store(false, Ordering::SeqCst);
        // Show cursor when exiting
        print!("{}", termion::cursor::Show);
        std::io::stdout().flush().unwrap();
    }).expect("Failed to set Ctrl-C handler.");

    // Hide cursor.
    print!("{}", termion::cursor::Hide);

    // Prepare the space for the animation.
    for _ in 0..args.height {
        println!();
    }

    // Return the cursor to the top of the canvas.
    print!("{}", termion::cursor::Up(args.height as u16));

    // Remember the start cursor position.
    let origin = 
    std::io::stdout().into_raw_mode().expect("Failed to switch to raw mode.")
        .cursor_pos().expect("Failed to get cursor position.");
    
    // Animate.
    let mut last_frame = Instant::now();

    let mut i = 0;
    while r.load(Ordering::SeqCst) {
        let frame = &frames[i];

        if last_frame.elapsed() < frame.dalay_as_duration() {
            thread::sleep(frame.dalay_as_duration() - last_frame.elapsed());
        }
        last_frame = Instant::now();
        
        print!("{}", termion::cursor::Goto(origin.0, origin.1));
        (args.printer.unwrap())(args, &frame.image);
        i = (i + 1) % frames.len();

    }

    print!("{}", termion::cursor::Show);
    std::io::stdout().flush().unwrap();
}