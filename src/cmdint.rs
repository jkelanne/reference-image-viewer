
use std::collections::HashMap;

//use crate::ref_image_view::*;
pub use crate::images::*;

pub type Handler<'a, T> = fn(&mut T, Option<String>) -> Option<i32>;

pub struct CommandParser<'a, T> {
	cmd_map: HashMap<&'a str, Handler<'a, T>>,
}

impl<'a, T> Default for CommandParser<'a, T> {
	fn default() -> Self {
		CommandParser {
			cmd_map: HashMap::new(),
		}
	}
}

impl<'a, T> CommandParser<'a, T> {
	//pub fn register(&mut self, cmd: &'a str, handler: fn(&'a str) -> std::option::Option<i32>) {
	pub fn register(&mut self, cmd: &'a str, handler: fn(&mut T, Option<String>) -> std::option::Option<i32>) {
		self.cmd_map.insert(cmd, handler);
	}

	//pub fn execute(&self, text: &'a std::string::String) {
	pub fn execute(&self, context: &mut T, text: Box<std::string::String>) {
		//let test_vec = text.collect::<Vec<&str>>();
		//let split_text = text.split(' ').collect::<Vec<&str>>();
		println!("TRYRING TO EXECUTE: [{:?}]", text);

		let split_text = text.split(' ').collect::<Vec<&str>>();
		println!("TRIED TO SPLIT! GOT: {:?}", split_text);

		if self.cmd_map.contains_key(split_text[0]) {
			println!("still???");
			if split_text.len() == 1 {
				self.cmd_map[split_text[0]](context, None);	
			} else {
				self.cmd_map[split_text[0]](context, Some(String::from(split_text[1])));	
			}
			
		}


		//if self.cmd_map.contains_key(split_text[0]) {
		//	self.cmd_map[split_text[0]](split_text[1]);
		//}
	}
}