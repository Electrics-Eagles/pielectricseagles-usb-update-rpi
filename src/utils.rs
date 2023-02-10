use std::fs;

/// Loads the contents of a file as a string.
///
/// # Arguments
///
/// * `filename` - The path to the file to be loaded, as a string slice.
///
/// # Returns
///
/// The contents of the file as a `String`, or an empty string `"[[]]"` if the file cannot be read.
///
/// # Example
///
/// ```
/// let contents = load_file_str("file.txt");
/// println!("{}", contents);
/// ```
pub fn load_file_str(filename: &str) -> String {
    let contents = match fs::read_to_string(filename.to_string()) {
        Ok(c) => c,
        Err(_) => "[[]]".to_string(),
    };
    contents
}
