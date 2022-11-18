use egui_extras::RetainedImage;
use eframe::egui;
//use std::thread;
use std::sync::mpsc;

pub use crate::utils::*;
pub use crate::images::*;
pub use crate::app_config::*;

pub struct RefImageView {
    images: Images,
    rx: mpsc::Receiver<(RetainedImage, std::string::String, std::string::String)>,
    image_scale: f32,
    auto_resize: bool,
    app_config: AppConfig,
}

impl RefImageView {
    pub fn new(app_config: AppConfig, cc: &eframe::CreationContext<'_>, rx: mpsc::Receiver<(RetainedImage, std::string::String, std::string::String)>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        //let images = Images::new(im_vec, hash_vec);

        let im_vec: Vec<RetainedImage> = Vec::new();
        let hash_vec: Vec<String> = Vec::new();
        let filenames: Vec<String> = Vec::new();
        let images = Images::new(im_vec, hash_vec, filenames);

        Self {
            images,
            rx,
            image_scale: 1.0,
            auto_resize: false,
            app_config,
        }  
    }
}

impl eframe::App for RefImageView {
    fn on_close_event(&mut self) -> bool {
        // Save the tags
        std::fs::write(get_tags_filename(), serde_json::to_string_pretty(&self.images.tags).unwrap(),).unwrap();

        // Save the AppConfig
        std::fs::write(get_conf_filename(), serde_json::to_string_pretty(&self.app_config).unwrap()).unwrap();

        return true;
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let incoming_image = self.rx.try_recv();
        match incoming_image {
            Ok(v) => {
                let (img, hash, filename) = v;
                self.images.images.push(img);
                self.images.hashes.push(hash);
                self.images.filenames.push(filename);
            },
            Err(_) => ()
        }
        egui::TopBottomPanel::top("rev_top_panel").show(ctx, |ui| {
            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                //println!("IS IT HAPPENING?!?!?");
                self.images.next();
            }

            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                //println!("IS IT HAPPENING?!?!?");
                self.images.prev();
            }

            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::I) {
                println!("Image Info:");
                println!("\tFilename: {}", self.images.filenames[self.images.index]);
                println!("\tHash: {}", self.images.get_current_image_hash());

                let tags: String = match self.images.get_current_image_tags() {
                    Some(vector) => vector.join("; "),
                    None => "No Tags!".to_string(),
                };
                println!("\tTags: {}", tags);
            }

            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::E) {
                /*let some_stuff = self.images.tags.iter().filter_map(|(key, &val)| if val == vec!["touhou".to_string(), "marisa".to_string()] { Some(key) } else { None })
                .collect::<Vec<_>>();*/
                for (key, value) in &self.images.tags {
                    if value.contains(&"touhou".to_string()) {
                        println!("{}", key);    
                    }
                }
                
            }

            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::T) {
                if !self.images.tags.contains_key(&self.images.get_current_image_hash()) {
                    self.images.tags.insert(self.images.get_current_image_hash(), vec!["gura".to_string()]);
                }
            }

            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
                /*println!("ESC ESC ESC");*/
                frame.close();
            }

            ui.horizontal(|ui| {
                if ui.button("<").clicked() {
                    self.images.prev();
                }
                if ui.button(">").clicked() {
                    self.images.next();
                }
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

                let i_size = self.images.get_size_of_current();
                let image_ratio = i_size[0] as f32 / i_size[1] as f32;


                if panel_ratio > image_ratio {
                    self.image_scale = ui.available_height() / i_size[1] as f32;
                } else {
                    self.image_scale = ui.available_width() / i_size[0] as f32;
                }
            }

            if self.images.has_images() {
                self.images.get(self.images.index).unwrap().show_scaled(ui, self.image_scale);    
            }
        });

        egui::TopBottomPanel::bottom("rev_bottom_panel").show(ctx, |ui| {
            ui.label(egui::RichText::new(format!("[image {}/{}]", (self.images.index + 1), self.images.images.len())));
        });

        let size = ctx.available_rect();
        // 340, 380
        // open(&mut true)
        // TODO: Need a way to get the window size so that we can freeze the windows
        // to the bottom right corner.

/*        let mut tags: String = String::new();
        let values = self.images.get_current_image_tags();*/

        /*match values {
            Err(v) => write!(tags, "No", v),
            Ok(v) => {
                for value in v {
                    //tags.push_str(value.to_string());
                    write!(tags, "{}; ", value).unwrap();
                }
            }
        };*/

        //write!(tags, "{:?}", values.unwrap());
        // let stuff_str: String = stuff.into_iter().map(|i| i.to_string()).collect::<String>();
        // vector.into_iter().map(|i| i.to_string()).collect::<String>(),

        let tags: String = match self.images.get_current_image_tags() {
            Some(vector) => vector.join("; "),
            None => "No Tags!".to_string(),
        };

        egui::Window::new("Test")
            .fixed_pos((size.width() - 240.0, size.height() - 70.0))
            .show(ctx, |ui| {
/*                ui.label(
                    egui::RichText::new(self.images.get_current_image_hash()),
                );*/
                ui.label(format!("{}", self.images.filenames[self.images.index]));
                ui.label(format!("current_size {:?}", ui.available_size()));
                ui.label(
                    egui::RichText::new(tags),
                );
        });
    }
}