use std::{fs::File, io::BufReader, path::Path};

use image::{codecs::gif::GifDecoder, io::Reader, AnimationDecoder, Delay, DynamicImage, ImageDecoder, ImageError};

pub struct GifFrame {
    pub image: DynamicImage, 
    pub delay: Delay,
}

impl GifFrame {
    pub fn dalay_as_duration(&self) -> std::time::Duration {
        let (num, denum) = self.delay.numer_denom_ms();
        std::time::Duration::from_millis((num / denum) as u64)
    }
}

pub enum ImageFile {
    Image(DynamicImage), Gif(Vec<GifFrame>, (u32, u32))
}

impl ImageFile {
    pub fn open<P>(path: P) -> Result<Self, ImageError>
    where P: AsRef<Path> {
        let reader = Reader::open(&path)?.with_guessed_format()?;
        match reader.format().expect("Invalid format.") {
            image::ImageFormat::Gif => {
                drop(reader); // make sure the file is not locked
                let gif_file = BufReader::new(File::open(&path)?);
                let decoder = GifDecoder::new(gif_file).unwrap();
                let dims = decoder.dimensions();
                let frames = decoder.into_frames().collect_frames()?;
                let frames = frames.into_iter().map(|f| {
                    GifFrame{
                        delay: f.delay(),
                        image: DynamicImage::ImageRgba8(f.into_buffer()),
                    }
                }).collect();

                Ok(ImageFile::Gif(frames, dims))
            },
            _ => {
                Ok(ImageFile::Image(reader.decode()?))
            }
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            ImageFile::Image(image) => (image.width(), image.height()),
            ImageFile::Gif(_, dims) => *dims,
        }
    }
}