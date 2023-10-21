use image::DynamicImage;
use rayon::prelude::*;
use std::error::Error;
use std::fmt;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppError {}

fn is_greyscale(image: &DynamicImage) -> bool {
    let rgb_image = image.to_rgb8();
    let pixels = rgb_image.pixels();
    for pixel in pixels {
        if pixel[0] != pixel[1] || pixel[0] != pixel[2] {
            return false;
        }
    }
    true
}

pub fn process_image(input_path: &Path, override_flag: bool) -> Result<(), Box<dyn Error>> {
    let image = image::open(input_path)?;
    let output_image;

    let greyscale = is_greyscale(&image);
    if greyscale {
        let mut gray_image = image.to_luma8();
        for pixel in gray_image.pixels_mut() {
            if pixel[0] > 220 {
                pixel[0] = 255;
            }
        }
        output_image = image::DynamicImage::ImageLuma8(gray_image);
    } else {
        let mut rgb_image = image.to_rgb8();
        for pixel in rgb_image.pixels_mut() {
            if pixel[0] > 220 {
                pixel[0] = 255;
            }
            if pixel[1] > 220 {
                pixel[1] = 255;
            }
            if pixel[2] > 220 {
                pixel[2] = 255;
            }
        }
        output_image = image::DynamicImage::ImageRgb8(rgb_image);
    }

    let parent_dir = input_path.parent().unwrap_or_else(|| Path::new(""));
    let file_stem = input_path
        .file_stem()
        .ok_or("Failed to get file stem")?
        .to_string_lossy();
    let extension = input_path
        .extension()
        .ok_or("Failed to get file extension")?
        .to_string_lossy();

    let output_path = if override_flag {
        input_path.to_path_buf()
    } else {
        parent_dir.join(format!("w_{}.{}", file_stem, extension))
    };

    output_image.save(output_path)?;
    Ok(())
}

pub fn process_directory(input_path: &Path, override_flag: bool) -> Result<(), AppError> {
    if input_path.is_dir() {
        let paths: Vec<_> = WalkDir::new(input_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.path().to_owned())
            .collect();
        paths.par_iter().for_each(|path| {
            if let Err(e) = process_image(path, override_flag) {
                eprintln!("Failed to process {}: {}", path.display(), e);
            }
        });
    } else if input_path.is_file() {
        if let Err(e) = process_image(input_path, override_flag) {
            eprintln!("Failed to process {}: {}", input_path.display(), e);
        }
    } else {
        return Err(AppError {
            message: format!("Invalid input path: {}", input_path.display()),
        });
    }
    Ok(())
}
