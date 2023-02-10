use sysinfo::{ProcessExt, System, SystemExt};
/// this function is check the pielectricseagles processes running and kills all of them.
/// Also prints out of OS where this process had been running.
pub fn check_task_is_running() {
    let s = System::new_all();
    for process in s.processes_by_exact_name("pielectricseagles") {
        let version_os = match s.long_os_version() {
            Some(version) => version,
            None => "N/A".to_string(),
        };
        process.kill();
        println!(
            "{},{}",
            "The PiElectrics eagles binary been running.... Killing it.:", version_os
        );
    }
}
