use std::fs;
use std::thread;
use std::time::Duration;

use chrono::Local;
use xcap::image::ImageBuffer;
use xcap::Window;
use clap::Parser;
use image::RgbaImage; // Import RgbaImage directly

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target window title (contains)
    #[arg(short, long, default_value = "Chrome")]
    window_title: String,

    /// Output directory for screenshots
    #[arg(short, long, default_value = "screenshots")]
    output_dir: String,

    /// Capture interval in seconds
    #[arg(short, long, default_value = "1")]
    interval: u64,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = fs::create_dir_all(&args.output_dir) {
        eprintln!("Error creating directory: {}", e);
        return;
    }

    loop {
        let window = find_window(&args.window_title);
        if window.is_none() {
            println!("Window '{}' not found. Exiting.", args.window_title);
            break;
        }
        let window = window.unwrap();

        if let Err(e) = capture_and_save_screenshot(&window, &args.output_dir) {
            eprintln!("Error capturing or saving screenshot: {}", e);
        }

        thread::sleep(Duration::from_secs(args.interval));
    }
}

fn find_window(title_contains: &str) -> Option<Window> {
    Window::all()
        .unwrap()
        .iter()
        .find(|window| window.title().contains(title_contains))
        .cloned()
}

fn capture_and_save_screenshot(window: &Window, target_dir: &str) -> Result<(), String> {
    let screenshot: ImageBuffer<xcap::image::Rgba<u8>, Vec<u8>> = window.capture_image().map_err(|e| e.to_string())?;

    let width = screenshot.width();
    let height = screenshot.height();
    let pixels = screenshot.into_raw();

    // Create RgbaImage directly
    let dynamic_image = match RgbaImage::from_raw(width, height, pixels) {
        Some(img) => image::DynamicImage::ImageRgba8(img),
        None => return Err("Failed to create RgbaImage".to_string()),
    };

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let filename = format!("{}/screenshot_{}.webp", target_dir, timestamp);

    dynamic_image
        .save_with_format(&filename, image::ImageFormat::WebP)
        .map_err(|e| e.to_string())?;

    Ok(())
}