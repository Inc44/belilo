use image::DynamicImage;
use rayon::prelude::*;
use std::error::Error;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
const WHITEN_THRESHOLD: u8 = 220;
const WHITEN_TARGET: u8 = 255;
fn is_grayscale(image: &DynamicImage) -> bool {
    image
        .to_rgb8()
        .pixels()
        .all(|pixel| pixel[0] == pixel[1] && pixel[0] == pixel[2])
}
fn whiten_grayscale(image: DynamicImage) -> DynamicImage {
    let mut grayscale_image = image.to_luma8();
    for pixel in grayscale_image.pixels_mut() {
        if pixel[0] > WHITEN_THRESHOLD {
            pixel[0] = WHITEN_TARGET;
        }
    }
    DynamicImage::ImageLuma8(grayscale_image)
}
fn whiten_rgb(image: DynamicImage) -> DynamicImage {
    let mut rgb_image = image.to_rgb8();
    for pixel in rgb_image.pixels_mut() {
        for channel in pixel.0.iter_mut() {
            if *channel > WHITEN_THRESHOLD {
                *channel = WHITEN_TARGET;
            }
        }
    }
    DynamicImage::ImageRgb8(rgb_image)
}
fn whiten_image(image: DynamicImage) -> DynamicImage {
    if is_grayscale(&image) {
        whiten_grayscale(image)
    } else {
        whiten_rgb(image)
    }
}
fn build_output_path(input_path: &Path, override_flag: bool) -> Result<PathBuf, Box<dyn Error>> {
    if override_flag {
        return Ok(input_path.to_path_buf());
    }
    let file_stem = input_path
        .file_stem()
        .ok_or("Failed to get file stem")?
        .to_string_lossy();
    let extension = input_path
        .extension()
        .ok_or("Failed to get file extension")?
        .to_string_lossy();
    let parent_dir = input_path.parent().unwrap_or_else(|| Path::new(""));
    Ok(parent_dir.join(format!("w_{}.{}", file_stem, extension)))
}
pub fn process_image(input_path: &Path, override_flag: bool) -> Result<(), Box<dyn Error>> {
    let image = image::open(input_path)?;
    let output_image = whiten_image(image);
    let output_path = build_output_path(input_path, override_flag)?;
    output_image.save(output_path)?;
    Ok(())
}
fn collect_paths(input_path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    if !input_path.is_dir() {
        return Err(format!("Invalid input path: {}", input_path.display()).into());
    }
    let paths = WalkDir::new(input_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.path().to_owned())
        .collect();
    Ok(paths)
}
pub fn process_images(input_path: &Path, override_flag: bool) -> Result<(), Box<dyn Error>> {
    if input_path.is_file() {
        if let Err(error) = process_image(input_path, override_flag) {
            eprintln!("Failed to process {}: {}", input_path.display(), error);
        }
        return Ok(());
    }
    let paths = collect_paths(input_path)?;
    paths.par_iter().for_each(|path| {
        if let Err(error) = process_image(path, override_flag) {
            eprintln!("Failed to process {}: {}", path.display(), error);
        }
    });
    Ok(())
}