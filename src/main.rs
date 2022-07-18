//use std::env;
use eframe::egui;
use std::{path, fs};
use std::slice::SliceIndex;
use egui_extras::RetainedImage;
use clap::{Arg, Command}; // , Parser
//use assets_manager::{Asset, AssetCache, loader};
//extern crate directories;
use directories::{UserDirs, ProjectDirs};

//use image;

// We'll call this Images for now.. ImageCache, though it's not really a cache?
pub struct Images {
    // Holds the images. Going with pub for now..
    pub images: Vec<RetainedImage>,
    // Holds the current index
    pub index: usize,
}

impl Images {
    pub fn new(images: Vec<RetainedImage>) -> Self {
/*        let index = match images.len() {
            0 => None,
            _ => Some(0),
        };*/

        Self {
            images,
            index: 0
        }
    }

    fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[RetainedImage]>,
    {
        self.images.get(index)
    }

    fn get_size_of_current(&self) -> [usize; 2] {
        self.images[0].size()
    }

    fn has_images(&self) -> bool {
        if self.images.len() > 0 {
            return true;
        }
        false
    }

    fn next(&mut self) {
        println!("self.index: {}; self.images.len: {}", self.index, self.images.len());
        if self.index < (self.images.len() - 1) {
            self.index += 1;
        }
    }

    fn prev(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }
}

struct RefImageView {
    //image: RetainedImage,
    images: Images,
    image_scale: f32,
    auto_resize: bool,
    //cache: AssetCache,
}

/*impl Default for RefImageView {
    fn default() -> Self {
        /*let mut cache: AssetCache;
        if let Some(proj_dirs) = ProjectDirs::from("com", "null ptr", "refiv") {
            if !path::Path::new(proj_dirs.config_dir()).exists() {
                fs::create_dir_all(proj_dirs.config_dir());
            }
            cache = AssetCache::new(proj_dirs.config_dir()).unwrap();
        }*/
        let proj_dirs = ProjectDirs::from("com", "null ptr", "refiv").unwrap();
        println!("Config dirs: {:?}; exists: [{}]", proj_dirs.config_dir(), path::Path::new(proj_dirs.config_dir()).exists());
        println!("Cache dirs: {:?}; exists: [{}]", proj_dirs.cache_dir(), path::Path::new(proj_dirs.cache_dir()).exists());
        println!("Data dirs: {:?}; exists: [{}]", proj_dirs.data_dir(), path::Path::new(proj_dirs.data_dir()).exists());
        // Config dirs: "C:\\Users\\jukka\\AppData\\Roaming\\null ptr\\refiv\\config"
        // Cache dirs: "C:\\Users\\jukka\\AppData\\Local\\null ptr\\refiv\\cache"
        // Data dirs: "C:\\Users\\jukka\\AppData\\Roaming\\null ptr\\refiv\\data"
        // }
/*        Self {
            image: RetainedImage::from_image_bytes(
                "FXm37GQaMAAAYs7.png",
                include_bytes!("../resources/FXm37GQaMAAAYs7.png"),
            ).unwrap(),
            image_scale: 0.25,
            auto_resize: false, 
            //ache: AssetCache::new(proj_dirs.config_dir()).unwrap()
        }*/
    }   
}*/

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
    fn new(cc: &eframe::CreationContext<'_>, images: Images) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        //Self::default()
        Self {
/*            image: RetainedImage::from_color_image(
                "filename",
                ci,
            ),*/
            images,
            image_scale: 1.0,
            auto_resize: false,
            //cache: AssetCache::new(proj_dirs.config_dir()).unwrap()
        }  
    }

/*    /*fn new_with_colorimage(cc: &eframe::CreationContext<'_>, ci: egui::ColorImage, scale: f32) -> Self {*/
    fn new_with_colorimage(cc: &eframe::CreationContext<'_>, ci: Images, scale: f32) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let proj_dirs = ProjectDirs::from("com", "null ptr", "refiv").unwrap();

        Self {
            image: RetainedImage::from_color_image(
                "filename",
                ci,
            ),
            image_scale: scale,
            auto_resize: false,
            //cache: AssetCache::new(proj_dirs.config_dir()).unwrap()
        }    
    }*/
}

impl eframe::App for RefImageView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        /*
            pub keys_down: HashSet<Key>,
            pub events: Vec<Event>,
        */

        egui::TopBottomPanel::top("rev_top_panel").show(ctx, |ui| {
            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                println!("IS IT HAPPENING?!?!?");
                self.images.next();
            }

            if ui.input_mut().consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                println!("IS IT HAPPENING?!?!?");
                self.images.prev();
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
                //println!("ui width: {}; height: {}", ui.available_width(), ui.available_height());
                let i_size = self.images.get_size_of_current();
                let image_ratio = i_size[0] as f32 / i_size[1] as f32;

                //println!("panel_ratio: {}; image_ratio: {}", panel_ratio, image_ratio);

                if panel_ratio > image_ratio {
                    self.image_scale = ui.available_height() / i_size[1] as f32;
                } else {
                    self.image_scale = ui.available_width() / i_size[0] as f32;
                }
                //println!("self.image_scale: {}", self.image_scale);
            }
            /*
                RetainedImage::from_color_image(
                    "filename",
                    ci,
                )
            */
            //let ni = RetainedImage::from_color_image("filename", self.images.get(0).unwrap());
            //ni.show(ui);
            if self.images.has_images() {
                self.images.get(self.images.index).unwrap().show_scaled(ui, self.image_scale);    
            }
            //self.image.get(0).show_scaled(ui, self.image_scale);
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

    if let Some(proj_dirs) = ProjectDirs::from("com", "null ptr", "refiv") {
        if !path::Path::new(proj_dirs.config_dir()).exists() {
            fs::create_dir_all(proj_dirs.config_dir());
        }

        println!("Config dirs: {:?}; exists: [{}]", proj_dirs.config_dir(), path::Path::new(proj_dirs.config_dir()).exists());
        println!("Cache dirs: {:?}; exists: [{}]", proj_dirs.cache_dir(), path::Path::new(proj_dirs.cache_dir()).exists());
        println!("Data dirs: {:?}; exists: [{}]", proj_dirs.data_dir(), path::Path::new(proj_dirs.data_dir()).exists());
        // Config dirs: "C:\\Users\\jukka\\AppData\\Roaming\\null ptr\\refiv\\config"
        // Cache dirs: "C:\\Users\\jukka\\AppData\\Local\\null ptr\\refiv\\cache"
        // Data dirs: "C:\\Users\\jukka\\AppData\\Roaming\\null ptr\\refiv\\data"
    }

    let options = eframe::NativeOptions::default();
    let in_file = matches.value_of("INPUT");
    //let mut ci: Option<egui::ColorImage> = None;

    let mut im_vec: Vec<RetainedImage> = Vec::new();

    match in_file {
        None => { 
            // Don't really have to do anything here..
            // println!("You need to enter a file with file argument!");
        },
        Some(s) => {
            let path = std::path::Path::new(s);

            if path.is_file() {
                println!("INPUT is a file..");
                let ri = RetainedImage::from_color_image("filename", load_image_from_path(path).unwrap());
                //im_vec.push(load_image_from_path(path).unwrap());
                im_vec.push(ri);

                /* 
                    image: RetainedImage::from_color_image(
                        "filename",
                        ci,
                    ),*/

                //let ci = load_image_from_path(std::path::Path::new(s)).unwrap();
                //ci = Some(load_image_from_path(path).unwrap());
                //match ci {
                //    None => {},
                //    Some(image) => im_vec.push(image),
                //}
                //im_vec.push(ci);
            } else {
                println!("INPUT is a directory..");
                for entry in path.read_dir().expect("read_dir call failed") {
                    if let Ok(entry) = entry {
                        // So with this we can load image(s) from a directory. Now we just need to figure out how to handle the files
                        // Also.. this is a stupid way of doing thinghs. we should not create the window inside these conditinals. Instead
                        // we should just load the files here and then load the window after images have been loaded.
                        //println!("{:?}", entry.path());

                        //ci = Some(load_image_from_path(entry.path().as_path()).unwrap());
                        let ri = RetainedImage::from_color_image("filename", load_image_from_path(entry.path().as_path()).unwrap());
                        im_vec.push(ri);
                        //im_vec.push(load_image_from_path(entry.path().as_path()).unwrap());
                    }
                }
            }
        }
    }

    let images = Images::new(im_vec);
    eframe::run_native(
        "Reference Image Viewer",
        options,
        Box::new(|cc| Box::new(RefImageView::new(cc, images))),
    );
/*    match ci {
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
    }*/
}