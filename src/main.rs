use belilo::process_images;
use clap::Parser;
use std::path::PathBuf;
use std::process;
#[derive(Parser)]
#[command(
    version,
    about = "A command-line tool for whitening images.",
    long_about = "Belilo, which translates to 'whitewasher' in Russian, is a useful tool created with ❤️ using Rust. It quickly whitens images, providing a clean, uniform appearance. It's fast, efficient, and precise.",
    override_usage = "belilo <input_paths>... [options]"
)]
struct Cli {
    /// Paths to the input images or directories (required)
    #[arg(value_name = "input_paths", required = true)]
    input_paths: Vec<PathBuf>,
    /// Override the input image instead of creating a new one
    #[arg(short, long = "override")]
    override_flag: bool,
    /// Keep modification time
    #[arg(short, long = "keep")]
    keep_flag: bool,
    /// Threshold for whitening (0-255) (default: 220)
    #[arg(
        short,
        long,
        value_name = "value",
        default_value_t = 220,
        hide_default_value = true
    )]
    threshold: u8,
}
fn main() {
    let cli = Cli::parse();
    let mut has_errors = false;
    for path in &cli.input_paths {
        if process_images(path, cli.override_flag, cli.keep_flag, cli.threshold).is_err() {
            has_errors = true;
        }
    }
    if has_errors {
        process::exit(1);
    }
}