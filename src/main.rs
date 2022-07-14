use eframe::egui;

struct RefImageView {
    name: String,
    age: u32,
}

impl Default for RefImageView {
    fn default() -> Self {
        Self {
            name: "Jukka".to_owned(),
            age: 42,
        }
    }   
}

impl eframe::App for RefImageView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RefImageView");
            ui.horizontal(|ui| {
                ui.label("Hello World: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click here").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello {}", "world"));
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Reference Image Viewer",
        options,
        Box::new(|_cc| Box::new(RefImageView::default())),
    );
}