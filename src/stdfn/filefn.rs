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
	Ok(file.split_terminator("\n").any(|m| m == string))
}

///converts a file to a list and adds an item to it
pub fn add_to_file(path: String, string: String) -> Result<()> {
	let mut text = str_from_file(path.clone())?;
	text = format!("{}\n{}", text, string);
	fs::write(path.clone(), text)
}

///returns a path to the file in the database associated with a given guild id
pub fn guild_file(id: u64, file: &str) -> String {
	format!("data/{}/{}.txt", id, file)
}

///returns a path to the file in the database for the given channel id
pub fn voting_channel_file(guild: u64, channel: u64, file: &str) -> String {
	format!("data/{}/votes/{}/{}.txt", guild, channel, file)
}

///creates a general directory
pub fn make_dir(path: String) -> Result<()> {
	let dir = DirBuilder::new();
	dir.create(path)
}

///creates a folder at the path specified for the given guild
pub fn create_dir(id: u64, path: String) -> Result<()> {
	make_dir(format!("data/{}/{}", id, path))
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
	fs::write(path.clone(), text)
}

///copies file from files to the folder of a specified guild
pub fn copy_file(id: u64, file: &str) -> Result<()> {
	let text = str_from_file(format!("files/{}.txt", file))?;
	write_to_file(guild_file(id, file), text)?;
	Ok(())
}