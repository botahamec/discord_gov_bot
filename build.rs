use std::process::Command;
use std::env;

fn main() {

    let out_dir = env::var("OUT_DIR").unwrap(); //the directory the project is being exported to
    
    Command::new("cp").arg(&format!("src/files"))
                        .arg(&format!("{}/files", out_dir))
                        .status().unwrap();
}