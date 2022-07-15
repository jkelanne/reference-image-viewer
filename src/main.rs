use std::env;
use eframe::egui;
use egui_extras::RetainedImage;
//use image;

struct RefImageView {
    image: RetainedImage,
    image_scale: f32,
    auto_resize: bool,
}

impl Default for RefImageView {
    fn default() -> Self {
        Self {
            image: RetainedImage::from_image_bytes(
                "FXm37GQaMAAAYs7.png",
                include_bytes!("../resources/FXm37GQaMAAAYs7.png"),
            ).unwrap(),
            image_scale: 0.25,
            auto_resize: false, 
        }
    }   
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

impl RefImageView {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }

    fn new_with_colorimage(cc: &eframe::CreationContext<'_>, ci: egui::ColorImage, scale: f32) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            image: RetainedImage::from_color_image(
                "filename",
                ci,
            ),
            image_scale: scale,
            auto_resize: false,
        }    
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

                if ui.button("*").clicked() {
                    self.auto_resize = true;
                }
                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.auto_resize {
                let panel_ratio = ui.available_width() / ui.available_height();
                //println!("ui width: {}; height: {}", ui.available_width(), ui.available_height());
                let i_size = self.image.size();
                let image_ratio = i_size[0] as f32 / i_size[1] as f32;

                //println!("panel_ratio: {}; image_ratio: {}", panel_ratio, image_ratio);

                if panel_ratio > image_ratio {
                    self.image_scale = ui.available_height() / i_size[1] as f32;
                } else {
                    self.image_scale = ui.available_width() / i_size[0] as f32;
                }
                //println!("self.image_scale: {}", self.image_scale);
            }
            self.image.show_scaled(ui, self.image_scale);
        });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let options = eframe::NativeOptions::default();

    if args.len() > 1 {
        if std::path::Path::new(&args[1]).exists() {
            // Bake error handling into load_image_from_path()
            let ci = load_image_from_path(std::path::Path::new(&args[1])).unwrap();

            eframe::run_native(
                "Reference Image Viewer",
                options,
                Box::new(|cc| Box::new(RefImageView::new_with_colorimage(cc, ci, 0.5))),
            );
        }    
    } else {
        eframe::run_native(
            "Reference Image Viewer",
            options,
            Box::new(|cc| Box::new(RefImageView::new(cc))),
        );
    }
}