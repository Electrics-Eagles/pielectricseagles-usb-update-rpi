/// path where  is config of updater is stored
const PATH_OF_CONFIG: &str = "/etc/pielectricseagles/updates/updater.toml";
use crate::{
    install_updates_files::update_files,
    kill_task::check_task_is_running,
    post_update_marker::{is_been_updated, post_update_marker},
    run_scripts::{run_scripts_after, run_scripts_before},
    unpack_zip::unpack_zip,
    update_json::parsed_updates,
    utils::load_file_str,
    validate_update_files::get_encrypted_zip_file_validation,
};
use std::{thread, time};
use toml::Value;
/// struct of TrustedDevice is toml config file
#[derive(Debug, Clone)]
struct TrustedDevice {
    vid: String,
    pid: String,
}
/// struct for  mountpath is toml config file
#[derive(Debug, PartialEq)]
struct MountPath {
    path: String,
}

/// the function that parses out of toml string ( so the file converted in string ) in to vec of trusted devices structs
fn parse_trusted_usb_devices(toml: &str) -> Result<Vec<TrustedDevice>, String> {
    let value = toml.parse::<Value>().map_err(|e| e.to_string())?;
    let devices_table = value
        .get("trusted_devices")
        .ok_or_else(|| "Error: 'devices' table not found".to_string())?;

    let mut devices = vec![];
    for device_entry in devices_table.as_array().unwrap() {
        let device_map = device_entry.as_table().unwrap();
        let vid = device_map
            .get("vid")
            .ok_or_else(|| "Error: 'vid' not found".to_string())?
            .as_str()
            .unwrap()
            .to_string();
        let pid = device_map
            .get("pid")
            .ok_or_else(|| "Error: 'pid' not found".to_string())?
            .as_str()
            .unwrap()
            .to_string();
        devices.push(TrustedDevice { vid, pid });
    }

    Ok(devices)
}
/// simple function that parses mount point where usb stick is mounted
/// todo two functions for same stuff one private one public ... common ... are you a devil ?
pub fn parse_mount_path(toml: &str) -> String {
    let value = toml.parse::<Value>().unwrap();
    let mount_path = value.get("mount_path");
    let path = mount_path.unwrap().get("path").unwrap().as_str();
    path.unwrap().to_string()
}
/// function that returns as result path of mounted stick.
/// todo two functions for same stuff one private one public ... common ... are you a devil ?
pub fn return_usb_stick_mount_path() -> String {
    let input_file_str = load_file_str(PATH_OF_CONFIG);
    let path = parse_mount_path(&input_file_str);
    return path;
}

/// function that gets tuple of u16 as input and
/// This function searches for a match between a given vendor id and product id
/// (`vid_pid`) and the trusted USB devices listed in a configuration file
/// (`PATH_OF_CONFIG`). If a match is found, the function initializes an update
/// procedure. This includes checking if a task is already running, sleeping for
/// five seconds, validating the encrypted update ZIP file, unpacking the ZIP
/// file, and updating the files.
pub fn found_match_vid_pid(vid_pid: (u16, u16)) {
    let input_file_str = load_file_str(PATH_OF_CONFIG);
    let trusted_devices = parse_trusted_usb_devices(&input_file_str);
    let trusted_devices_result = match trusted_devices {
        Ok(trusted_devices) => trusted_devices,
        Err(_) => todo!(),
    };
    for item in &trusted_devices_result {
        let vid_u16 = u16::from_str_radix(&item.vid, 16).unwrap();
        let pid_u16 = u16::from_str_radix(&item.pid, 16).unwrap();
        if vid_u16 == vid_pid.0 && pid_u16 == vid_pid.1 {
            println!("============================> Update <=========================");
            check_task_is_running();
            thread::sleep(time::Duration::from_millis(5000));
            let is_valid_files = get_encrypted_zip_file_validation();
            if !is_been_updated() {
                if is_valid_files {
                    unpack_zip(vid_pid);
                    let updates = parsed_updates();
                    run_scripts_before();
                    update_files(updates);
                    run_scripts_after();
                    post_update_marker();
                    print!("\x1B[2J\x1B[1;1H");
                    println!("======================> Update OK <===============");
                }
            } else {
                println!("{}", "Error the file is on stick ... update files on usb");
            }
        } else {
            println!(
                "For current vid:{} and pid:{},there no match found in config.",
                vid_pid.0, vid_pid.1
            );
        }
    }
}
