use crate::validate_update_files::get_zip_path;

/// Unpack the encrypted zip file to a given directory
///
/// # Arguments
/// * `vid_pid` - A tuple of vendor id and product id in `u16` format.
///
/// # Example
/// ```
/// use crate::unpack_zip::unpack_zip;
/// unpack_zip((0x1234, 0x5678));
/// ```
pub fn unpack_zip(vid_pid: (u16, u16)) {
    let password = format!("{}{}", vid_pid.0, vid_pid.1);
    let zip_path = get_zip_path();
    let password_str = password.as_str();
    sevenz_rust::decompress_file_with_password(
        zip_path,
        "/tmp/update/pielectricseagles/",
        password_str.into(),
    )
    .expect("complete");
}
