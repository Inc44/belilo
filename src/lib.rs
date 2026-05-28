use image::{ColorType, DynamicImage, GenericImageView};
use rayon::prelude::*;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;
use walkdir::WalkDir;
const WHITEN_TARGET: u8 = 255;
fn is_grayscale(image: &DynamicImage) -> bool {
    match image.color() {
        ColorType::L8 | ColorType::La8 | ColorType::L16 | ColorType::La16 => true,
        _ => image
            .pixels()
            .all(|(_, _, pixel)| pixel[0] == pixel[1] && pixel[0] == pixel[2]),
    }
}
fn whiten_grayscale(image: DynamicImage, threshold: u8) -> DynamicImage {
    let mut grayscale_image = image.into_luma8();
    for pixel in grayscale_image.pixels_mut() {
        if pixel[0] > threshold {
            pixel[0] = WHITEN_TARGET;
        }
    }
    DynamicImage::ImageLuma8(grayscale_image)
}
fn whiten_rgb(image: DynamicImage, threshold: u8) -> DynamicImage {
    let mut rgb_image = image.into_rgb8();
    for pixel in rgb_image.pixels_mut() {
        for channel in pixel.0.iter_mut() {
            if *channel > threshold {
                *channel = WHITEN_TARGET;
            }
        }
    }
    DynamicImage::ImageRgb8(rgb_image)
}
fn whiten_image(image: DynamicImage, threshold: u8) -> DynamicImage {
    if is_grayscale(&image) {
        whiten_grayscale(image, threshold)
    } else {
        whiten_rgb(image, threshold)
    }
}
fn format_file_name(file_stem: &str, extension: &str, counter: u32) -> String {
    let counter_str = if counter > 0 {
        format!("_{:03}", counter)
    } else {
        String::new()
    };
    let extension_str = if extension.is_empty() {
        String::new()
    } else {
        format!(".{}", extension)
    };
    format!("whitened_{}{}{}", file_stem, counter_str, extension_str)
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
    let output_path = (0..1000)
        .map(|counter| {
            let file_name = format_file_name(&file_stem, &extension, counter);
            parent_dir.join(file_name)
        })
        .find(|path| !path.exists())
        .ok_or("Failed to generate a unique output path")?;
    Ok(output_path)
}
pub fn process_image(
    input_path: &Path,
    override_flag: bool,
    keep_flag: bool,
    threshold: u8,
) -> Result<(), Box<dyn Error>> {
    let image = image::open(input_path)?;
    if image.width() == 0 || image.height() == 0 {
        return Ok(());
    }
    let mut modified_date = SystemTime::now();
    if keep_flag {
        let metadata = fs::metadata(input_path)?;
        modified_date = metadata.modified()?;
    }
    let output_path = build_output_path(input_path, override_flag)?;
    let whitened_image = whiten_image(image, threshold);
    whitened_image.save(&output_path)?;
    if keep_flag {
        filetime::set_file_times(&output_path, modified_date.into(), modified_date.into())?;
    }
    Ok(())
}
fn collect_paths(input_path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    if !input_path.is_dir() {
        eprintln!("Invalid input path: {}", input_path.display());
        return Err("".into());
    }
    let paths = WalkDir::new(input_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.path().to_owned())
        .collect();
    Ok(paths)
}
pub fn process_images(
    input_path: &Path,
    override_flag: bool,
    keep_flag: bool,
    threshold: u8,
) -> Result<(), Box<dyn Error>> {
    if input_path.is_file() {
        if let Err(error) = process_image(input_path, override_flag, keep_flag, threshold) {
            eprintln!("Failed to process {}: {}", input_path.display(), error);
            return Err("".into());
        }
        return Ok(());
    }
    let paths = collect_paths(input_path)?;
    let has_errors = AtomicBool::new(false);
    paths.par_iter().for_each(|path| {
        if let Err(error) = process_image(path, override_flag, keep_flag, threshold) {
            eprintln!("Failed to process {}: {}", path.display(), error);
            has_errors.store(true, Ordering::Relaxed);
        }
    });
    if has_errors.load(Ordering::Relaxed) {
        return Err("".into());
    }
    Ok(())
}