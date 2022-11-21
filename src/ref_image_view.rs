use egui_extras::RetainedImage;
use eframe::egui;
//use std::thread;
use std::sync::mpsc;

pub use crate::utils::*;
pub use crate::images::*;
pub use crate::app_config::*;
pub use crate::cmdint::*;

pub struct RefImageView<'a> {
    images: Images,
    rx: mpsc::Receiver<(RetainedImage, std::string::String, std::string::String)>,
    image_scale: f32,
    auto_resize: bool,
    app_config: AppConfig,
    display_keys: bool,
    display_tags: bool,
    display_cmd: bool,
    cmd_field: String,
    cmd_parser: CommandParser<'a, Images>,
}
/*
    fn test_handler(args: &str) -> Option<i32> {
        println!("Test handler!! GOT ARGS: {}", args);
        return Some(1337);
    }

    println!("#####################");

    let mut parser: CommandParser = CommandParser::default();
    parser.register("test_cmd", test_handler);
    parser.execute("test_cmd 111");

    println!("#####################");

    if !self.images.tags.contains_key(&self.images.get_current_image_hash()) {
        self.images.tags.insert(self.images.get_current_image_hash(), vec!["gura".to_string()]);
    } 
*/

fn tag_add(context: &mut Images, args: Option<String>) -> Option<i32> {
    println!("Test handler!! GOT ARGS: {}", args.as_ref().unwrap());
    println!("Current image index: {}", context.index);

    if !context.tags.contains_key(&context.get_current_image_hash()) {
        //context.images.tags.insert(context.images.get_current_image_hash(), vec![args.to_string()]);
        context.add_tag_to_current(args.unwrap());
    }

    return None;
}

fn tag_clear(context: &mut Images, _args: Option<String>) -> Option<i32> {
    println!("TAG CLEAR CALLED!");
    context.clear_tags_from_current();
    return None;
}

fn get_images_with_tag(context: &mut Images, args: Option<String>) -> Option<i32> {
    match args {
        Some(tags) => { 
            println!("get images with tag: {}", tags);
            context.get_images_with_tag(tags);
        }
        None => println!("No args!"),
    }

    return None;
}

impl<'a> RefImageView<'a> {
    pub fn new(app_config: AppConfig, cc: &eframe::CreationContext<'_>, rx: mpsc::Receiver<(RetainedImage, std::string::String, std::string::String)>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        //let images = Images::new(im_vec, hash_vec);

        let im_vec: Vec<RetainedImage> = Vec::new();
        let hash_vec: Vec<String> = Vec::new();
        let filenames: Vec<String> = Vec::new();
        let images = Images::new(im_vec, hash_vec, filenames);

        let mut cmd_parser: CommandParser<Images> = CommandParser::default();
        cmd_parser.register("tag_add", tag_add);
        cmd_parser.register("tag_clear", tag_clear);
        cmd_parser.register("get", get_images_with_tag);

        Self {
            images,
            rx,
            image_scale: 1.0,
            auto_resize: false,
            app_config,
            display_keys: false,
            display_tags: true,
            display_cmd: false,
            cmd_field: "type cmd".to_string(),
            cmd_parser,
        }  
    }
}

impl<'a> eframe::App for RefImageView<'a> {
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
                if !self.display_cmd {
                    //println!("IS IT HAPPENING?!?!?");
                    self.images.next();
                }
            }

            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                if !self.display_cmd {
                    //println!("IS IT HAPPENING?!?!?");
                    self.images.prev();
                }
            }

            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::I) {
                if !self.display_cmd {
                    println!("Image Info:");
                    println!("\tFilename: {}", self.images.filenames[self.images.index]);
                    println!("\tHash: {}", self.images.get_current_image_hash());

                    let tags: String = match self.images.get_current_image_tags() {
                        Some(vector) => vector.join("; "),
                        None => "No Tags!".to_string(),
                    };
                    println!("\tTags: {}", tags);
                }
            }

            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::E) {
                /*let some_stuff = self.images.tags.iter().filter_map(|(key, &val)| if val == vec!["touhou".to_string(), "marisa".to_string()] { Some(key) } else { None })
                .collect::<Vec<_>>();*/
                if !self.display_cmd {
                    for (key, value) in &self.images.tags {
                        if value.contains(&"touhou".to_string()) {
                            println!("{}", key);    
                        }
                    }
                }
            }

            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::F1) {
                // Don't know how to read key holding, so should do toggle for now..
                self.display_keys = match self.display_keys {
                    true => false,
                    false => true,
                }
            }

            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::F2) {
                // Don't know how to read key holding, so should do toggle for now..
                self.display_tags = match self.display_tags {
                    true => false,
                    false => true,
                }
            }

            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::T) {
                if !self.display_cmd {
                    if !self.images.tags.contains_key(&self.images.get_current_image_hash()) {
                        self.images.tags.insert(self.images.get_current_image_hash(), vec!["gura".to_string()]);
                    }                    
                }
            }

            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
                if !self.display_cmd {
                    frame.close();    
                } else {
                    self.display_cmd = false;
                }
                /*println!("ESC ESC ESC");*/
                
            }

            if ui.input_mut().consume_key(egui::Modifiers::CTRL, egui::Key::Space) {
                self.display_cmd = match self.display_cmd {
                    true => false,
                    false => true,
                }
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
            /*.fixed_pos((10.0, 10.0))*/
            
        let mut tags_window = egui::Window::new("Tags")
            .id(egui::Id::new("tags_window"))
            .title_bar(false)
            .resizable(true);

        if self.display_tags {
            tags_window.show(ctx, |ui| {
                ui.label("tags");
                match self.images.get_current_image_tags() {
                    Some(v) => {
                        for t in &v {
                            // .background_color(egui::Color32::LIGHT_BLUE)
                            // .color(egui::Color32::DARK_GRAY)
                            if ui.add(egui::widgets::Label::new(
                                egui::RichText::new(t)
                                    .background_color(egui::Color32::LIGHT_BLUE)
                                    .color(egui::Color32::DARK_RED)
                                    .monospace()
                            ).sense(egui::Sense::click())).clicked() {
                                println!("DOES THIS WORK?!?");
                            }
                        }
                    }, 
                    None => ()
                };
            });    
        }
        
        let hotkeys_window = egui::Window::new("Hotkeys")
            .id(egui::Id::new("hotkeys_window"))
            .title_bar(false);

        if self.display_keys {
            hotkeys_window.show(ctx, |ui| {
                ui.label("hotkeys");
                ui.label("F1 -- Display this menu");
                ui.label("F2 -- Display tags");
                ui.label("CTRL+Space -- Enter command");
            });
        }


        let cmd_window = egui::Window::new("Command")
            .id(egui::Id::new("cmd_window"))
            .title_bar(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO);
        if self.display_cmd {
            cmd_window.show(ctx, |ui| {
                 ui.horizontal(|ui| {
                    ui.label("cmd:");
                    let cmd = ui.add(egui::TextEdit::singleline(&mut self.cmd_field));
                    cmd.request_focus();
                    //if cmd.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    
                    if ui.input().key_pressed(egui::Key::Enter) {
                        println!("huh? {}", self.cmd_field);
                        //let s_slice: &str = &*self.cmd_field;
                        //let tmp_str = String::from(self.cmd_field.to_owned());
                        //let t = ;
                        self.cmd_parser.execute(&mut self.images, Box::new(String::from(self.cmd_field.as_str().to_owned())));
                        self.display_cmd = false;
                    }
                });
            });
        }
    }
}