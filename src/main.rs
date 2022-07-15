use eframe::egui;
use egui_extras::RetainedImage;
//use image;

struct RefImageView {
    image: RetainedImage,
    image_scale: f32,
}

impl Default for RefImageView {
    fn default() -> Self {
        Self {
            image: RetainedImage::from_image_bytes(
                "FXm37GQaMAAAYs7.png",
                include_bytes!("../resources/FXm37GQaMAAAYs7.png"),
            ).unwrap(),
            image_scale: 0.25,
        }
    }   
}

impl RefImageView {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }
}

impl eframe::App for RefImageView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("rev_top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("+").clicked() {
                    self.image_scale += 0.01;
                }

                if ui.button("-").clicked() {
                    self.image_scale -= 0.01;
                }
                egui::widgets::global_dark_light_mode_switch(ui);
/*                if ui.button("Dark").clicked() {
                    ctx.set_visuals(egui::Visuals::dark());
                }*/
            });
        });

/*        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.label("Hello World!");
        });*/

        egui::CentralPanel::default().show(ctx, |ui| {
            // self.image.show(ui);
/*            ui.horizontal(|ui| {
                if ui.button("+").clicked() {
                    self.image_scale += 0.01;
                }

                if ui.button("-").clicked() {
                    self.image_scale -= 0.01;
                }                
            });*/



            self.image.show_scaled(ui, self.image_scale);
            // ui.heading("RefImageView");

/*            ui.add(
                egui::Image::new(self.image.texture_id(ctx), self.image.size_vec2())
                    .rotate(45.0_f32.to_radians(), egui::Vec2::splat(0.5)),
            );*/

            /*ui.horizontal(|ui| {
                ui.label("Hello World: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click here").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello {}", "world"));*/
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Reference Image Viewer",
        options,
        Box::new(|cc| Box::new(RefImageView::new(cc))),
    );
}