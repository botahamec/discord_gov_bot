//stdlib imports
use std::{
	fs,
	fs::{
		File,
		DirBuilder,
	},
	io::{
		prelude::*,
		Result,
		Error,
		ErrorKind
	},
};

///returns a string from a file
pub fn str_from_file(path: String) -> Result<String> {
	let mut file = File::open(path)?;
	let mut string = String::new();
	file.read_to_string(&mut string)?;
	Ok(string)
}

pub fn vec_from_file(path: String) -> Result<Vec<String>> {
	let file = str_from_file(path)?;
	let mut vec : Vec<String> = file.split_terminator("\n").map(|s| String::from(s)).collect();
	if vec.len() > 1 && vec[0] == String::from("") {vec.remove(0);}
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

///removes an item from a file
pub fn remove_from_file(path: String, string: String) -> Result<()> {
	let mut list = vec_from_file(path.clone())?;
	for item in 0..list.len() {
		if list[item] == string {list.remove(item);}
	}
	let text : String = list.join("\n");
	write_to_file(path.clone(), text)?;
	Ok(())
}

///returns a path to the file in the database associated with a given guild id
pub fn guild_file(id: u64, file: &str) -> String {
	format!("data/servers/{}/{}.txt", id, file)
}

///returns a path to the file in the database for the given channel id
pub fn voting_channel_file(guild: u64, channel: u64, file: &str) -> String {
	format!("data/servers/{}/votes/{}/{}.txt", guild, channel, file)
}

///creates a general directory
pub fn make_dir(path: String) -> Result<()> {
	let dir = DirBuilder::new();
	dir.create(path)
}

///creates a folder at the path specified for the given guild
pub fn create_dir(id: u64, path: String) -> Result<()> {
	make_dir(format!("data/servers/{}/{}", id, path))
}

///creates a file for the guild specified with the given path
pub fn create_file(id: u64, file: &str) -> Result<()> {
	File::create(format!("data/servers/{}/{}.txt", id, file))?;
	Ok(())
}

///creates a file for the guild specified with the given path
pub fn create_vote_channel_file(guild: u64, channel: u64, file: &str) -> Result<()> {
	File::create(format!("data/servers/{}/votes/{}/{}.txt", guild, channel, file))?;
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

pub fn channel_from_abbr(guild: u64, abbr: String) -> Result<u64> {
	let paths = fs::read_dir(format!("data/servers/{}/votes", guild))?;
	for path in paths {
		let channel_str = format!("{}", path.unwrap().path().file_name().unwrap().to_str().unwrap());
		let test_abbr = str_from_file(format!("data/servers/{}/votes/{}/abbr.txt", guild, channel_str))?;
		if test_abbr == abbr {
			let channel_id = channel_str.parse::<u64>().unwrap();
			return Ok(channel_id);
		}
	}
	Err(Error::new(ErrorKind::InvalidInput, "No channel found with this abbreviation"))
}