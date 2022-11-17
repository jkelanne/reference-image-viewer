//use std::env;
use eframe::egui;
use std::{path, fs};
use std::collections::HashMap;
use std::slice::SliceIndex;
use std::thread;
use std::sync::mpsc;
use std::fs::File;
use std::io::BufReader;
use std::fmt::Write;
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


fn get_tags_filename() -> String {
    let mut tags_file: String = String::new();
    if let Some(proj_dirs) = ProjectDirs::from("com", "null ptr", "refiv") {
        tags_file = proj_dirs.data_dir().to_owned().into_os_string().into_string().unwrap();
        tags_file.push_str("\\tags.json");
    }
    tags_file
}

// We'll call this Images for now.. ImageCache, though it's not really a cache?
pub struct Images {
    // Holds the images. Going with pub for now..
    pub images: Vec<RetainedImage>,
    pub hashes: Vec<String>,
    pub filenames: Vec<String>,
    // Holds the current index
    pub index: usize,
    pub tags: HashMap<String, Vec<String>>,
}

impl Images {
    pub fn new(images: Vec<RetainedImage>, hashes: Vec<String>, filenames: Vec<String>) -> Self {
        let tags_file = get_tags_filename();
        let mut tags: HashMap<String, Vec<String>> = HashMap::new();
        if path::Path::new(&tags_file).exists() {
            let tags_raw = File::open(tags_file).unwrap();
            let tags_reader = BufReader::new(tags_raw);
            tags = serde_json::from_reader(tags_reader).unwrap();
        }
        
        Self {
            images,
            hashes,
            filenames,
            index: 0,
            tags,
            //tags: HashMap::new()
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

    // Somekind of error handling please.. should return Result<Vec<String>> or something
    fn get_current_image_tags(&self) -> Option<Vec<String>> {
        if self.tags.len() == 0 {
            return None;
        }

        //let mut result = vec![];
        if self.tags.contains_key(&self.get_current_image_hash()) {
            return Some(self.tags[&self.get_current_image_hash()].to_vec());
        } else {
            return None;
        }
    }
}

// We should add checksums for loaded images so that we don't have to generate them again
struct RefImageView {
    images: Images,
    rx: mpsc::Receiver<(RetainedImage, std::string::String, std::string::String)>,
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
    fn new(cc: &eframe::CreationContext<'_>, rx: mpsc::Receiver<(RetainedImage, std::string::String, std::string::String)>) -> Self {
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
        }  
    }
}

impl eframe::App for RefImageView {
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
        let json_exists = std::path::Path::new(&data_file).exists();
        println!("json_exists: {:?}; data_file:  {}", json_exists, data_file);

        let cache_raw = File::open(data_file).unwrap();
        let cache_reader = BufReader::new(cache_raw);
        let cache: HashMap<String, Vec<String>> = serde_json::from_reader(cache_reader).unwrap();
        //println!("SERRRRRDE:\n{:?}",cache);

        for(key, value) in &cache {
            println!("\t{:?}: {:?}", key, value);
        }
    }

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let in_file = matches.value_of("INPUT");
        let mut file_count = 0;
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
                    tx.send((ri, hash, String::from(path.to_str().unwrap()))).unwrap();
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
                                // This is really ugly..
                                tx.send((ri, hash, String::from(entry.path().to_str().unwrap()))).unwrap();
                                file_count += 1;
                            }
                        }
                    }
                }
            }
        }

        println!("### IMAGES LOADED! TOTAL: {} ###", file_count);
    });

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Reference Image Viewer",
        options,
        Box::new(|cc| Box::new(RefImageView::new(cc, rx))),
    );
}