mod after_callback;
mod install_updates_files;
mod kill_task;
mod mount_disk;
mod post_update_marker;
mod run_scripts;
mod unpack_zip;
mod update_json;
mod utils;
mod validate_update_files;
extern crate hex;
use crate::after_callback::found_match_vid_pid;
use rusb::{Context, Device, HotplugBuilder, UsbContext};
struct HotPlugHandler;

/// A function that takes in a `Params` struct and a tuple of two strings, and outputs the contents of the tuple to the console.
///
/// # Parameters
///
/// * `device` - A tuple of two strings that represent the vendor and product codes of a device.
fn callback(device: (u16, u16)) {
    found_match_vid_pid(device);
}
/// A function that is called when a device is connected.
///
/// The function takes a `Device` as input and retrieves its vendor and product codes.
/// If the retrieval of the device descriptor succeeds, the vendor and product codes are extracted.
/// If the retrieval of the device descriptor fails, a default vendor and product code of "0000,0000" are used.
/// The extracted or default vendor and product codes are then formatted into hexadecimal strings.
/// Finally, the `callback` function is called with the `params` struct and the formatted vendor and product codes as inputs.
///
impl<T: UsbContext> rusb::Hotplug<T> for HotPlugHandler {
    fn device_arrived(&mut self, device: Device<T>) {
        let device_descriptor_result = device.device_descriptor();
        let vendor_product_code = match device_descriptor_result {
            Ok(result) => (result.vendor_id(), result.product_id()),
            Err(_result) => (0, 0),
        };
        callback((vendor_product_code.0, vendor_product_code.1));
    }

    fn device_left(&mut self, device: Device<T>) {
        println!("update stick seems  left {:?}", device);
    }
}
/// Defines an implementation of the `Drop` trait for the `HotPlugHandler` type.
///
/// The `Drop` trait is a special trait in Rust that is automatically called when an object goes out of scope.
/// This implementation of the `Drop` trait ensures that when a `HotPlugHandler` object is no longer in use,
/// a message will be printed to indicate that it has been dropped. This can be useful for debugging or for
/// keeping track of the lifetime of objects in your program.
impl Drop for HotPlugHandler {
    /// The implementation of the `drop` method for the `HotPlugHandler` type.
    ///
    /// This method is called automatically when a `HotPlugHandler` object goes out of scope and will execute the code inside the method.
    /// In this case, the method will print the message "HotPlugHandler dropped" to the console.
    fn drop(&mut self) {
        println!("HotPlugHandler dropped");
    }
}

/// Callback function for handling USB connections
///
/// This function creates a new USB context, and registers a hotplug handler. The hotplug handler is responsible for detecting
/// USB connections and disconnections. The function returns an error if it fails to create the context or register the hotplug handler.
/// If successful, the function enters a loop that handles USB events and waits for the hotplug handler to be unregistered.
///
fn callback_usb_connected() -> rusb::Result<()> {
    let context = Context::new()?;
    let mut reg = Some(
        HotplugBuilder::new()
            .enumerate(true)
            .register(&context, Box::new(HotPlugHandler {}))?,
    );

    Ok(loop {
        context.handle_events(None).unwrap();
        if let Some(reg) = reg.take() {
            context.unregister_callback(reg);
            break;
        }
    })
}

fn main() -> rusb::Result<()> {
    if rusb::has_hotplug() {
        loop {
            let result = callback_usb_connected();
            drop(result);
        }
    } else {
        eprint!("libusb hotplug api unsupported");
        Ok(())
    }
}
