use screenshots::Screen;
use image::{ImageBuffer, RgbaImage};
// use std::fs;
use leptess::{LepTess, Variable};

fn main() {
    let screens = Screen::all().unwrap(); // Get all screens
    let screen = &screens[0]; // Select the first screen

    let image = screen.capture().unwrap(); // Take a screenshot

    // Convert the screenshot to an RgbaImage
    let img: RgbaImage = ImageBuffer::from_raw(image.width(), image.height(), image.into_raw()).unwrap();

    let path = "screenshot.png";
    img.save(path).unwrap(); // Save as PNG

    println!("📸 Screenshot saved as {}", path);


/* -----------------------------------------------------
                    SCREENSHOT CODE     
--------------------------------------------------------
 */

    // Step 2: Perform OCR (Extract text from image)
    let mut ocr = LepTess::new(None, "eng").unwrap(); // Load Tesseract with English language
    ocr.set_variable(Variable::TesseditPagesegMode, "6").unwrap(); // Set OCR mode
    ocr.set_image(path).unwrap(); // Load the screenshot

    let text = ocr.get_utf8_text().unwrap();
    println!("📝 Extracted Text: \n{}", text);
}