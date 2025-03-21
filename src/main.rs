use screenshots::Screen;
use image::{ImageBuffer, RgbaImage};

fn main() {
    let screens = Screen::all().unwrap(); // Get all screens
    let screen = &screens[0]; // Select the first screen

    let image = screen.capture().unwrap(); // Take a screenshot

    // Convert the screenshot to an RgbaImage
    let img: RgbaImage = ImageBuffer::from_raw(image.width(), image.height(), image.into_raw()).unwrap();

    let path = "screenshot.png";
    img.save(path).unwrap(); // Save as PNG

    println!("📸 Screenshot saved as {}", path);
}
