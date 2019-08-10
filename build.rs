use std::process::Command;
use std::env;

#[cfg(target_os="linux")]
fn copy(out_dir: String) {
	Command::new("cp").arg(&format!("src/files"))
                        .arg(&format!("{}/files", out_dir))
                        .status().unwrap();
}

#[cfg(target_os="windows")]
fn copy(out_dir: String) {
	Command::new("xcopy").arg(&format!("src/files"))
                        .arg(&format!("{}/files", out_dir))
                        .status().unwrap();
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap(); //the directory the project is being exported to
    println!("{}", out_dir);
    copy(out_dir);
}