use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::slice::SliceIndex;
use std::collections::HashMap;
use egui_extras::RetainedImage;

//mod utils;
pub use crate::utils::*;

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
        if Path::new(&tags_file).exists() {
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

    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[RetainedImage]>,
    {
        self.images.get(index)
    }

    pub fn get_size_of_current(&self) -> [usize; 2] {
        self.images[self.index].size()
    }

    pub fn has_images(&self) -> bool {
        if self.images.len() > 0 {
            return true;
        }
        false
    }

    pub fn next(&mut self) {
        if self.index < (self.images.len() - 1) {
            self.index += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn get_current_image_hash(&self) -> String {
        if self.hashes.len() > 0 {
            return String::from(&self.hashes[self.index]);    
        }
        return String::from("");
    }

    // Somekind of error handling please.. should return Result<Vec<String>> or something
    pub fn get_current_image_tags(&self) -> Option<Vec<String>> {
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