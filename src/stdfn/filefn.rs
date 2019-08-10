//stdlib imports
use std::{
	fs,
	fs::{
		File,
		DirBuilder
	},
	io::{
		prelude::*,
		Result,
	},
	process::Command,
};

///returns a string from a file
fn str_from_file(path: String) -> Result<String> {
	let mut file = File::open(path)?;
	let mut string = String::new();
	file.read_to_string(&mut string)?;
	Ok(string)
}

pub fn vec_from_file(path: String) -> Result<Vec<String>> {
	let file = str_from_file(path)?;
	let vec : Vec<String> = file.split_terminator("\n").map(|s| String::from(s)).collect();
	Ok(vec)
}

///turns a file into a list of strings and sees if it contains a ceratin value
pub fn file_contains(path: String, string: String) -> Result<bool> {
	let file = str_from_file(path)?;
	let boolean = file.split_terminator("\n").any(|m| m == string);
	Ok(boolean)
}

///converts a file to a list and adds an item to it
pub fn add_to_file(path: String, string: String) -> Result<()> {
	let mut file = File::open(path.clone())?;
	let mut text = String::new();
	file.read_to_string(&mut text)?;
	text = format!("{}\n{}", text, string);
	fs::write(path.clone(), text)?;
	Ok(())
}

///returns a path to the file in the database associated with a given guild id
pub fn guild_file(id: u64, file: &str) -> String {
	format!("data/{}/{}.txt", id, file)
}

pub fn voting_channel_file(guild: u64, channel: u64, file: &str) -> String {
	format!("data/{}/voting_channels/{}/{}.txt", guild, channel, file)
}

pub fn make_dir(path: String) -> Result<()> {
	let dir = DirBuilder::new();
	dir.create(path)?;
	Ok(())
}

///creates a folder at the path specified for the given guild
pub fn create_dir(id: u64, path: String) -> Result<()> {
	make_dir(format!("data/{}/{}", id, path))?;
	Ok(())
}

///creates a file for the guild specified with the given path
pub fn create_file(id: u64, file: &str) -> Result<()> {
	File::create(format!("data/{}/{}.txt", id, file))?;
	Ok(())
}

///creates a file for the guild specified with the given path
pub fn create_vote_channel_file(guild: u64, channel: u64, file: &str) -> Result<()> {
	File::create(format!("data/{}/votes/{}/{}.txt", guild, channel, file))?;
	Ok(())
}

pub fn write_to_file(path: String, text: String) -> Result<()> {
	fs::write(path.clone(), text)?;
	Ok(())
}

///copies file from files to the folder of a specified guild
#[cfg(target="linux")]
pub fn copy_file(id: u64, file: &str) {
	Command::new("cp")
		.arg(format!("files/{}.txt", file))
		.arg(format!("data/{}/{}.txt", id, file))
		.status().unwrap();
}

#[cfg(target_os="windows")]
pub fn copy_file(id: u64, file: &str) {
	Command::new("copy").arg(&format!("files/{}.txt", file))
                        .arg(&format!("data/{}/{}.txt", id, file))
                        .status().unwrap();
}