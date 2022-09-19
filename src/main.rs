//use std::env;
use eframe::egui;
use std::{path, fs};
use std::collections::HashMap;
use std::slice::SliceIndex;
use std::thread;
use std::sync::mpsc;
//use std::hash::{Hash, Hasher};
use egui_extras::RetainedImage;
use clap::{Arg, Command}; // , Parser
//use assets_manager::{Asset, AssetCache, loader};
//extern crate directories;
use directories::{UserDirs, ProjectDirs};
// use sha256::digest_bytes;

use serde::{Deserialize, Serialize};
use serde_json::Result;

struct MetaData {

}

// We'll call this Images for now.. ImageCache, though it's not really a cache?
pub struct Images {
    // Holds the images. Going with pub for now..
    pub images: Vec<RetainedImage>,
    pub hashes: Vec<String>,
    // Holds the current index
    pub index: usize,
    pub tags: HashMap<String, Vec<String>>,
}

impl Images {
    pub fn new(images: Vec<RetainedImage>, hashes: Vec<String>) -> Self {
        Self {
            images,
            hashes,
            index: 0,
            tags: HashMap::new()
        }
    }

    fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[RetainedImage]>,
    {
        self.images.get(index)
    }

    fn get_size_of_current(&self) -> [usize; 2] {
        self.images[self.index].size()
    }

    fn has_images(&self) -> bool {
        if self.images.len() > 0 {
            return true;
        }
        false
    }

    fn next(&mut self) {
        if self.index < (self.images.len() - 1) {
            self.index += 1;
        }
    }

    fn prev(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    fn get_current_image_hash(&self) -> String {
        if self.hashes.len() > 0 {
            return String::from(&self.hashes[self.index]);    
        }
        return String::from("");
    }
}

// We should add checksums for loaded images so that we don't have to generate them again
struct RefImageView {
    images: Images,
    rx: mpsc::Receiver<(RetainedImage, std::string::String)>,
    image_scale: f32,
    auto_resize: bool,
}

fn load_image_from_path(path: &std::path::Path) -> 
    (std::result::Result<egui::ColorImage, image::ImageError>, String) {
    let image = image::io::Reader::open(path).unwrap().decode().unwrap();
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    let hash1 = blake3::hash(image.as_bytes());

    (
        Ok(egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        )),
        String::from(hash1.to_hex().as_str())
    )
}

impl RefImageView {
    fn new(cc: &eframe::CreationContext<'_>, rx: mpsc::Receiver<(RetainedImage, std::string::String)>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        //let images = Images::new(im_vec, hash_vec);

        let mut im_vec: Vec<RetainedImage> = Vec::new();
        let mut hash_vec: Vec<String> = Vec::new();
        let mut images = Images::new(im_vec, hash_vec);

        Self {
            images,
            rx,
            image_scale: 1.0,
            auto_resize: false,
        }  
    }
}

impl eframe::App for RefImageView {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let incoming_image = self.rx.try_recv();
        match incoming_image {
            Ok(v) => {
                let (img, hash) = v;
                self.images.images.push(img);
                self.images.hashes.push(hash);
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

        let size = ctx.available_rect();
        // 340, 380
        // open(&mut true)
        // TODO: Need a way to get the window size so that we can freeze the windows
        // to the bottom right corner.
        egui::Window::new("Test")
            .fixed_pos((size.width() - 240.0, size.height() - 70.0))
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new(self.images.get_current_image_hash()),
                );
                ui.label(format!("available_size {:?}", size));
                ui.label(format!("current_size {:?}", ui.available_size()))
        });
    }
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    std::path::Path::new(filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
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

    let mut data_file: String = String::new();
    if let Some(proj_dirs) = ProjectDirs::from("com", "null ptr", "refiv") {
        if !path::Path::new(proj_dirs.config_dir()).exists() {
            fs::create_dir_all(proj_dirs.config_dir()).expect("Unable to create config dir");
        }

        if !path::Path::new(proj_dirs.data_dir()).exists() {
            fs::create_dir_all(proj_dirs.data_dir()).expect("Unable to create data dir");
        }

        println!("Config dirs: {:?}; exists: [{}]", proj_dirs.config_dir(), path::Path::new(proj_dirs.config_dir()).exists());
        println!("Cache dirs: {:?}; exists: [{}]", proj_dirs.cache_dir(), path::Path::new(proj_dirs.cache_dir()).exists());
        println!("Data dirs: {:?}; exists: [{}]", proj_dirs.data_dir(), path::Path::new(proj_dirs.data_dir()).exists());
        // Config dirs: "C:\\Users\\jukka\\AppData\\Roaming\\null ptr\\refiv\\config"
        // Cache dirs: "C:\\Users\\jukka\\AppData\\Local\\null ptr\\refiv\\cache"
        // Data dirs: "C:\\Users\\jukka\\AppData\\Roaming\\null ptr\\refiv\\data"

        //data_file.push_str(proj_dirs.data_dir().to_owned().into_os_string().into_string().unwrap());
        data_file = proj_dirs.data_dir().to_owned().into_os_string().into_string().unwrap();
        data_file.push_str("\\tags.json");
        println!("{}", data_file);
    }

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let in_file = matches.value_of("INPUT");

        match in_file {
            None => { 
                // Don't really have to do anything here..
            },
            Some(s) => {
                let path = std::path::Path::new(s);

                if path.is_file() {
                    println!("INPUT is a file..");
                    let (ret_img, hash) = load_image_from_path(path);
                    let ri = RetainedImage::from_color_image("filename", ret_img.unwrap());
                    tx.send((ri, hash)).unwrap();
                } else {
                    println!("INPUT is a director.read_dir()y..");
                    for entry in path.read_dir().expect("read_dir call failed") {
                        if let Ok(entry) = entry {
                            // So with this we can load image(s) from a directory. Now we just need to figure out how to handle the files
                            // Also.. this is a stupid way of doing thinghs. we should not create the window inside these conditinals. Instead
                            // we should just load the files here and then load the window after images have been loaded.
                            if entry.path().is_file() {
                                let (ret_img, hash) = load_image_from_path(entry.path().as_path()); //.unwrap()
                                let ri = RetainedImage::from_color_image("filename", ret_img.unwrap());
                                tx.send((ri, hash)).unwrap();
                            }
                        }
                    }
                }
            }
        }
    });

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Reference Image Viewer",
        options,
        Box::new(|cc| Box::new(RefImageView::new(cc, rx))),
    );
}