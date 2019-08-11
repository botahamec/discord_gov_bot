use std::fs::File;
use std::fs::DirBuilder;
use std::fs;
use std::io::Result;
use std::io::Read;

///returns a string from a file
fn str_from_file(path: String) -> Result<String> {
	let mut file = File::open(path)?;
	let mut string = String::new();
	file.read_to_string(&mut string)?;
	Ok(string)
}

fn copy_file(out_dir: &str, file: &str) -> Result<()> {
	let text = str_from_file(format!("src/files/{}", file))?;
	fs::write(format!("{}/files/{}", out_dir, file), text)?;
	Ok(())
}

///creates a general directory
pub fn make_dir(path: &str) -> Result<()> {
	let dir = DirBuilder::new();
	dir.create(path)
}

fn main() -> Result<()> {
    match make_dir("target/debug/data") {
        Ok(_t) => _t,
        Err(_e) => print!("")
    };
    match make_dir("target/debug/files") {
        Ok(_t) => _t,
        Err(_e) => print!("")
    };
    copy_file("target/debug", "yeas.txt")?;
    copy_file("target/debug", "nays.txt")?;
    copy_file("target/debug", "abst.txt")?;
    copy_file("target/debug", ".key")?;
    Ok(())
}