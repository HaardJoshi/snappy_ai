use eframe::egui;
use screenshots::Screen;
use image::{GenericImageView, RgbaImage, ImageBuffer}; // Image imports
use leptess::{LepTess, Variable}; // OCR library
use std::sync::{Arc, Mutex};
use arboard::Clipboard;

struct App {
    screenshot_path: String,
    extracted_text: Arc<Mutex<String>>,
    user_query: String,
    texture: Option<egui::TextureHandle>,
}

impl App {
    fn new() -> Self {
        let screenshot_path = "screenshot.png".to_string();
        let extracted_text = Arc::new(Mutex::new("".to_string()));
        let texture = None;

        // Initially take a screenshot and extract text
        let text = take_screenshot_and_extract_text(&screenshot_path);
        *extracted_text.lock().unwrap() = text;

        Self {
            screenshot_path,
            extracted_text,
            user_query: "".to_string(),
            texture,
        }
    }

    // Function to load the image as texture for UI display
    fn load_image(&mut self, ctx: &egui::Context) {
        let img = image::open(self.screenshot_path.clone()).unwrap();
        let (width, height) = img.dimensions();
        let rgba = img.to_rgba8();

        // Upload the image to GPU as texture for efficient UI rendering
        self.texture = Some(ctx.load_texture(
            "screenshot_texture",
            egui::ColorImage::from_rgba_unmultiplied(
                [width as usize, height as usize],
                &rgba,
            ),
            egui::TextureOptions::default(),
        ));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Snappy AI - Screenshot OCR");

            // Screenshot Preview Button
            if ui.button("📸 Take Screenshot").clicked() {
                let text = take_screenshot_and_extract_text(&self.screenshot_path);
                *self.extracted_text.lock().unwrap() = text;
                self.load_image(ctx); // Reload the image texture after capturing
            }

            ui.separator();

            // Show screenshot preview if texture exists
            if let Some(texture) = &self.texture {
                ui.image(texture); // Display the texture (screenshot)
            }

            ui.separator();
            ui.label("📝 Extracted Text:");
            ui.text_edit_multiline(&mut *self.extracted_text.lock().unwrap());

            // Copy to Clipboard Button
            if ui.button("📋 Copy Text").clicked() {
                let mut clipboard = Clipboard::new().unwrap();
                clipboard.set_text(self.extracted_text.lock().unwrap().clone()).unwrap();
            }

            ui.separator();
            ui.label("💬 Ask AI a Question:");
            ui.text_edit_singleline(&mut self.user_query);

            // Send to AI Button
            if ui.button("🤖 Ask AI").clicked() {
                let extracted_text = self.extracted_text.lock().unwrap().clone();
                let response = send_to_ai(&extracted_text, &self.user_query);
                *self.extracted_text.lock().unwrap() = format!("🤖 AI Response:\n{}", response);
            }
        });
    }
}

// Function to capture screenshot and extract text using OCR
fn take_screenshot_and_extract_text(path: &str) -> String {
    let screens = Screen::all().unwrap();
    let screen = &screens[0];
    let image = screen.capture().unwrap();

    // Save the captured screenshot to file
    let img: RgbaImage = ImageBuffer::from_raw(image.width(), image.height(), image.into_raw()).unwrap();
    img.save(path).unwrap();

    // OCR processing
    let mut ocr = LepTess::new(None, "eng").unwrap();
    ocr.set_variable(Variable::TesseditPagesegMode, "6").unwrap();
    ocr.set_image(path).unwrap();

    ocr.get_utf8_text().unwrap()
}

// Placeholder AI function to simulate asking questions to AI
fn send_to_ai(extracted_text: &str, user_query: &str) -> String {
    format!(
        "I received this:\nExtracted: {}\nQuestion: {}",
        extracted_text, user_query
    )
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Snappy AI",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    ).unwrap();
}