//use std::env;
use eframe::egui;
use egui_extras::RetainedImage;
use clap::{Arg, Command, Parser};
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
    let matches = Command::new("Reference Image Viewer")
        .author("Jukka Kelanne, jukka.kelanne@gmail.com")
        .version("0.1")
        .about("Lightweight (hopefully) image viewer")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .takes_value(true)
                .help("Start RIV with an image"))
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .takes_value(true)
                .help("Start RIV with a directory"))
        .arg(
            Arg::new("INPUT")
                //.about("Sets the input file")
                .required(false)
                .index(1))
        .get_matches();

    let options = eframe::NativeOptions::default();
    let in_file = matches.value_of("INPUT");
    let mut ci: Option<egui::ColorImage> = None;

    match in_file {
        None => { 
            // Don't really have to do anything here..
            // println!("You need to enter a file with file argument!");
        },
        Some(s) => {
            let path = std::path::Path::new(s);

            if path.is_file() {
                println!("INPUT is a file..");
                //let ci = load_image_from_path(std::path::Path::new(s)).unwrap();
                ci = Some(load_image_from_path(path).unwrap());
            } else {
                println!("INPUT is a directory..");
                for entry in path.read_dir().expect("read_dir call failed") {
                    if let Ok(entry) = entry {
                        // So with this we can load image(s) from a directory. Now we just need to figure out how to handle the files
                        // Also.. this is a stupid way of doing thinghs. we should not create the window inside these conditinals. Instead
                        // we should just load the files here and then load the window after images have been loaded.
                        println!("{:?}", entry.path());

                        ci = Some(load_image_from_path(entry.path().as_path()).unwrap());
                    }
                }
            }
        }
    }

    match ci {
        None => {
            eframe::run_native(
                "Reference Image Viewer",
                options,
                Box::new(|cc| Box::new(RefImageView::new(cc))),
            );
        },
        Some(i) => {
            eframe::run_native(
                "Reference Image Viewer",
                options,
                Box::new(|cc| Box::new(RefImageView::new_with_colorimage(cc, i, 0.5))),
            );
        }
    }
}