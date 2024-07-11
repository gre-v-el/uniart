use std::{fs::File, io::BufReader, path::Path};

use image::{codecs::gif::GifDecoder, io::Reader, AnimationDecoder, Delay, DynamicImage, ImageDecoder, ImageError};

pub struct GifFrame {
    pub image: DynamicImage, 
    pub delay: Delay,
    pub top: u32,
    pub left: u32,
}

pub enum ImageFile {
    Image(DynamicImage), Gif(Vec<GifFrame>, (u32, u32)), Video
}

impl ImageFile {
    pub fn open<P>(path: P) -> Result<Self, ImageError>
    where P: AsRef<Path> {
        let reader = Reader::open(&path)?.with_guessed_format()?;
        if let Some(format) = reader.format() {
            match format {
                image::ImageFormat::Gif => {
                    drop(reader); // make sure the file is not locked
                    let gif_file = BufReader::new(File::open(&path)?);
                    let decoder = GifDecoder::new(gif_file).unwrap();
                    let dims = decoder.dimensions();
                    let frames = decoder.into_frames().collect_frames()?;
                    let frames = frames.into_iter().map(|f| {
                        let delay = f.delay();
                        let top = f.top();
                        let left = f.left();
                    
                        GifFrame{
                            image: DynamicImage::ImageRgba8(f.into_buffer()),
                            delay,
                            top,
                            left,
                        }
                    }).collect();

                    Ok(ImageFile::Gif(frames, dims))
                },
                _ => {
                    Ok(ImageFile::Image(reader.decode()?))
                }
            }
        }
        else {
            // check for videos
            todo!()
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            ImageFile::Image(image) => (image.width(), image.height()),
            ImageFile::Gif(_, dims) => *dims,
            ImageFile::Video => todo!()
        }
    }
}