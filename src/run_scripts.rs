use std::process::Command;
/// Executes the shell script located at the given path.
///
/// # Arguments
///
/// * `path` - A string slice that represents the path to the shell script.
///
/// # Example
///
/// ```
/// use std::process::Command;
///
/// fn run_script(path: String) {
///     let output = Command::new("bash")
///         .arg(path.clone())
///         .output()
///         .expect("Failed to execute command");
///     let output_option = String::from_utf8(output.stdout.as_slice().to_vec());
///     let output_of_bash = match output_option {
///         Ok(output) => output,
///         Err(_output) => "Error in getting output".to_string(),
///     };
///     println!("{} output is {}:", path, output_of_bash);
/// }
/// ```
///
/// This function uses the `Command` struct from the `std::process` module to execute the shell script at the given path. The `output` method is used to get the output of the executed command, which is then converted to a UTF-8 string. The output string is then printed to the console.
fn run_script(path: String) {
    let output = Command::new("bash")
        .arg(path.clone())
        .output()
        .expect("Failed to execute command");
    let output_option = String::from_utf8(output.stdout.as_slice().to_vec());
    let output_of_bash = match output_option {
        Ok(output) => output,
        Err(_output) => "Error in getting output".to_string(),
    };
    println!("{} output is {}:", path, output_of_bash);
}

/// Executes the shell script located at the path "/tmp/update/pielectricseagles/encrypted_update/pre_update_script.sh".
///
/// # Example
///
/// ```
/// use std::process::Command;
///
/// pub fn run_scripts_before() {
///     run_script("/tmp/update/pielectricseagles/encrypted_update/pre_update_script.sh".to_string());
/// }
/// ```
///
/// This function calls the `run_script` function with the path "/tmp/update/pielectricseagles/encrypted_update/pre_update_script.sh".
pub fn run_scripts_before() {
    run_script("/tmp/update/pielectricseagles/encrypted_update/pre_update_script.sh".to_string());
}

/// Executes the shell script located at the path "/tmp/update/pielectricseagles/encrypted_update/post_update_script.sh".
///
/// # Example
///
/// ```
/// use std::process::Command;
///
/// pub fn run_scripts_after() {
///     run_script("/tmp/update/pielectricseagles/encrypted_update/post_update_script.sh".to_string());
/// }
/// ```
///
/// This function calls the `run_script` function with the path "/tmp/update/pielectricseagles/encrypted_update/post
pub fn run_scripts_after() {
    run_script("/tmp/update/pielectricseagles/encrypted_update/post_update_script.sh".to_string());
}
