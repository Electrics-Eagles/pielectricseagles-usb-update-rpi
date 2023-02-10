use std::{fs, path::Path};

use crate::{update_json::UpdateList, validate_update_files::validate_check_sum};
/// this function is update files with new files been specified in update.json
/// as input it takes and UpdateList struct. Also second parameter is an path where we expect to had files.
/// also this function checks and validates an check_sums and if they match updaste files. If they not match alarm message appears.
///
pub fn update_files(update_list: UpdateList) {
    let path_input = Path::new("/tmp/update/pielectricseagles/encrypted_update/update");
    for update in &update_list.update {
        let file_in_use = update.clone();
        let file_to_copy = path_input.join(file_in_use.location_file);
        let file_to_copy_checksum = file_in_use.checksum;
        let file_to_copy_string = file_to_copy.as_os_str().to_str().unwrap();
        if validate_check_sum(file_to_copy_string, file_to_copy_checksum) {
            println!(
                "checksum for file {}, is vaild and the checksum is {}",
                file_to_copy_string, "aa"
            );
            fs::copy(file_to_copy_string, update.install_path.clone()).unwrap();
        } else {
            eprintln!("Invaild checksum for file:{}", file_to_copy_string);
        }
    }
}
