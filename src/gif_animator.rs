use std::{io::Write, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread, time::Instant};

use crossterm::ExecutableCommand;

use crate::{image_file::GifFrame, Args};

pub fn animate_gif(args: &Args, frames: &Vec<GifFrame>) {
    // Make sure the cursor is shown when the program exits.
    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);

    ctrlc::set_handler(move || {
        running.store(false, Ordering::SeqCst);
        // Show cursor when exiting
        let mut stdout = std::io::stdout();
        stdout.execute(crossterm::cursor::Show).expect("Failed to show cursor.");
        stdout.flush().unwrap();
    }).expect("Failed to set Ctrl-C handler.");

    let mut stdout = std::io::stdout();
    // Hide cursor.
    stdout.execute(crossterm::cursor::Hide).expect("Failed to hide cursor.");

    // Prepare the space for the animation.
    for _ in 0..args.height {
        println!();
    }

    // Return the cursor to the top of the canvas.
    stdout.execute(crossterm::cursor::MoveUp(args.height as u16)).expect("Failed to restore cursor position.");

    // Remember the start cursor position.
    let origin = crossterm::cursor::position().expect("Failed to get cursor position.");
    
    // Animate.
    let mut last_frame = Instant::now();

    let mut i = 0;
    while r.load(Ordering::SeqCst) {
        let frame = &frames[i];

        if last_frame.elapsed() < frame.dalay_as_duration() {
            thread::sleep(frame.dalay_as_duration() - last_frame.elapsed());
        }
        last_frame = Instant::now();
        
        stdout.execute(crossterm::cursor::MoveTo(origin.0, origin.1)).expect("Failed to move cursor.");
        (args.printer.unwrap())(args, &frame.image);
        i = (i + 1) % frames.len();

    }

    stdout.execute(crossterm::cursor::Show).expect("Failed to show cursor.");
    std::io::stdout().flush().unwrap();
}