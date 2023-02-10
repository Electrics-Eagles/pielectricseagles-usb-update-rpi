use serde::{Deserialize, Serialize};

use crate::utils::load_file_str;

/// # Summary
/// The `parsed_updates` function reads and parses the update.json file from the specified path.
///
/// # Description
/// This function reads the contents of the update.json file as a string and then uses `serde_json` to parse the JSON data into a struct `UpdateList` which is defined in this module. The struct holds an array of structs `Update` that represent individual updates.
///
/// # Arguments
/// None
///
/// # Returns
/// This function returns an instance of `UpdateList` struct that holds an array of updates.
///
/// # Example
/// ```rust
/// use my_module::parsed_updates;
/// let updates = parsed_updates();
/// ```
pub fn parsed_updates() -> UpdateList {
    let json_file_as_string =
        load_file_str("/tmp/update/pielectricseagles/encrypted_update/update.json");
    let updates: UpdateList = serde_json::from_str(&json_file_as_string).unwrap();
    println!("{:?}", updates);
    return updates;
}

/// A struct that represents an update.
///
/// # Fields
///
/// * `location_file` - A string that represents the location of the file that needs to be updated.
/// * `checksum` - A string that represents the checksum of the file.
/// * `version` - A string that represents the version of the file.
/// * `install_path` - A string that represents the installation path of the file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Update {
    pub location_file: String,
    pub checksum: String,
    pub version: String,
    pub install_path: String,
}

/// A struct that holds an array of `Update` structs.
///
/// # Fields
///
/// * `update` - An array of `Update` structs.
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateList {
    pub update: Vec<Update>,
}
