# pielectricseagles-usb-update-rpi  
Simple tool that waits while correct usb stick is conencts copies files from it and run scripts. As part of pielectricseagles project.
The project is a Rust application that provides a callback function for handling USB connections. The main purpose of the project is to detect the arrival and departure of a USB device and take appropriate actions based on its vendor and product codes.

The project uses the rusb and hex libraries for USB functionality and hexadecimal string manipulation respectively. The main functionality of the project is implemented in the callback_usb_connected function which creates a new USB context and registers a hotplug handler. The hotplug handler detects USB connections and disconnections and calls the callback function with the vendor and product codes of the connected device.

The project contains several modules that handle different aspects of the USB update process, such as mount_disk, validate_update_files, unpack_zip, utils, update_json, install_updates_files, kill_task, run_scripts, and post_update_marker. These modules work together to ensure that the update process is smooth and error-free.

The main function calls the callback_usb_connected function in a loop, which listens for USB events and waits for the hotplug handler to be unregistered. The project is built to be flexible and can be easily extended to support additional functionality in the future.

# requriments
1.libusb  
2.udev  
3.bash  
4.rpi zero   
5. good mood  

# build
use cross 
```
cross build --target arm-unknown-linux-musleabi --release
```
# libs used

* hex: version 0.4.3 
* libusb1-sys: version 0.6.4 with the "vendored" feature 
* rusb: version 0.9.1 with the "vendored" feature 
* serde: version 1.0.152 with the "derive" feature 
* serde_json: version 1.0.93 
* sha256: version 1.0.3 
* sys-mount: version 2.0.2 
* toml: version 0.7.2 
* zip: version 0.6.4 
* sevenz-rust: version 0.2.3 with the "aes256" feature 
* sysinfo: version 0.27.7 

These libraries play important roles in the functioning of the pielectricseagles-usb-update For example, hex is used for hexadecimal encoding and decoding, libusb1-sys and rusb provide Rust bindings for libusb1, serde and serde_json are used for serializing and deserializing data, sha256 provides a secure hash algorithm, sys-mount provides an interface to mount and unmount file systems, toml is used for parsing and serializing TOML configuration files, zip provides support for creating and unpacking zip files, sevenz-rust is used to extract and create 7z archives and sysinfo provides access to system information. 


# installation 
0. get binaries from release 
1. create file in /etc/pielectricseagles/updater/updater.toml this is config file. Edit it for your situtation. 
2. add rule in udev that automaticly will mount an usb stick with correct vid and pid. 
3.using tool for pc perpare usb stick with all files requries.
4.do not forget about scripts that runs before and after installation.

# docs
docs are build and you can access them there:https://electrics-eagles.github.io/pielectricseagles-usb-update-rpi/pielectricseagles_usb_update/index.html
