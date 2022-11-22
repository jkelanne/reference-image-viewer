
use std::path::Path;
use eframe::egui;
use egui_extras::RetainedImage;

use std::{path, fs};

use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
use std::fs::File;
use std::io::{Write, BufReader};

use clap::{Arg, Command}; // , Parser
use directories::ProjectDirs;

//use serde::{Serialize, Deserialize};
/*use serde_derive::{Deserialize, Serialize};*/
//use serde_json::*;

mod utils;
mod images;
mod ref_image_view;
mod app_config;
mod cmdint;
pub use crate::utils::*;
pub use crate::images::*;
pub use crate::ref_image_view::*;
pub use crate::app_config::*;
pub use crate::cmdint::*;

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

    //let app_config = AppConfig::new(get_conf_filename());
    let mut app_config = {
        let cfn = get_conf_filename();
        if std::path::Path::new(&cfn).exists() {
            let conf_raw = std::fs::read_to_string(&cfn).unwrap();
            serde_json::from_str::<AppConfig>(&conf_raw).unwrap()
        } else {
            AppConfig::empty()
        }
    };

    println!("APP CONFIG CONTENT: {:?}", app_config);

    if let Some(proj_dirs) = ProjectDirs::from("com", "null ptr", "refiv") {
        if !std::path::Path::new(proj_dirs.config_dir()).exists() {
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

/*        let mut data_file: String = String::from(proj_dirs.data_dir().to_owned().into_os_string().into_string().unwrap());
        data_file.push_str("\\tags.json");*/
        //let json_exists = std::path::Path::new(&data_file).exists();
        let json_exists = std::path::Path::new(&get_tags_filename()).exists();
        if !json_exists {
            let mut file = std::fs::File::create(&get_tags_filename()).unwrap();
            file.write_all(b"{}").unwrap();
            //fs::write(data_file, b"{}");
        }
        println!("json_exists: {:?}; data_file:  {}", json_exists, get_tags_filename());

        let cache_raw = File::open(get_tags_filename()).unwrap();
        let cache_reader = BufReader::new(cache_raw);
        let cache: HashMap<String, Vec<String>> = serde_json::from_reader(cache_reader).unwrap();
        //println!("SERRRRRDE:\n{:?}",cache);

        for(key, value) in &cache {
            println!("\t{:?}: {:?}", key, value);
        }
    }

    let (tx, rx) = mpsc::channel();

    match matches.value_of("INPUT") {
        Some(v) => {
            println!("INPUT CONTAINS: [{}] Updating the last_open_location", v);
            app_config.last_open_location = String::from(v);
        } 
        None => println!("INPUT IS EMPTY"),
    }

    // need to have a movable version of the string last open location.
    let lol = String::from(app_config.last_open_location.clone());

    thread::spawn(move || {
        let in_file = match matches.value_of("INPUT") {
            Some(v) => Some(String::from(v)),
            None => Some(lol)
        };
        
/*        let mut file_count = 0;*/
        match in_file {
            None => { 
                // Don't really have to do anything here..
            },
            Some(s) => {
                let path = std::path::Path::new(&s);

                if path.is_file() {
                    println!("INPUT is a file..");
                    let (ret_img, hash) = load_image_from_path(path);
                    let ri = RetainedImage::from_color_image("filename", ret_img.unwrap());
                    tx.send((ri, hash, String::from(path.to_str().unwrap()))).unwrap();
                } else {
                    println!("INPUT is a director.read_dir()..");
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
/*                                file_count += 1;*/
                            }
                        }
                    }
                }
            }
        }

/*        println!("### IMAGES LOADED! TOTAL: {} ###", file_count);*/
    });

    let options = eframe::NativeOptions {
        decorated: true,
        transparent: true,
        min_window_size: Some(egui::vec2(320.0,100.0)),
        multisampling: 0,
        ..Default::default()
    };

    eframe::run_native(
        "Reference Image Viewer",
        options,
        Box::new(|cc| Box::new(RefImageView::new(app_config, cc, rx))),
    );
}