use std::{fs::File, io::Write, path::Path};

use crate::after_callback::return_usb_stick_mount_path;

#[derive(Debug)]
enum FileOutput {
    File(File),
    Error(()),
}

pub fn is_been_updated() -> bool {
    let get_path = return_usb_stick_mount_path();
    let path = Path::new(&get_path).join("update_sucess");
    if !path.exists() {
        return false;
    }
    println!("Update been completed. USB stick has marker file");
    return path.exists();
}
pub fn post_update_marker() {
    let get_path = return_usb_stick_mount_path();
    let path = Path::new(&get_path).join("update_sucess");
    let file_result = File::create(path);
    let mut file_result = match file_result {
        Ok(file) => file,
        Err(e) => return println!("{}", "error while createn file"),
    };
    let result = file_result.write_all(b"++UPDATE++");
    let result_output = match result {
        Ok(_) => println!("{}", "File is written fine"),
        Err(_) => println!("{}", "error while written file"),
    };
}
