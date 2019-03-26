// Crate for notifications
extern crate notify_rust;

use notify_rust::Notification;
use std::path::Path;
use std::process::Command;

fn main() {
    upcheck();
}

fn upcheck() {
    // Declare variables
    let checkupdates = "/usr/bin/checkupdates";

    // check if pacman-contrib and libnotify are installed
    if Path::new(&checkupdates).exists() {
        // Get output of checkupdates command
        let available_updates = Command::new(&checkupdates)
            .output()
            .expect("Failed to execute the checkupdates command.");

        // if the previous command exit with sucess, continue.
        if available_updates.status.success() {
            // Convert the output of the command into a string.
            let available_updates = String::from_utf8(available_updates.stdout)
                .expect("Output of checkupdates could not be parsed as valid UTF-8.");
            // Send the notification if updates are or not available
            if &available_updates.len() > &0 {
                Notification::new()
                    .summary("The following updates are available:")
                    .body(&available_updates)
                    .icon("dialog-information")
                    .show()
                    .unwrap();
            } else {
                Notification::new()
                    .summary("No updates are available.")
                    .icon("dialog-information")
                    .show()
                    .unwrap();
            }
        } else {
            eprintln!("Process exited with: {}", available_updates.status);
            eprintln!(
                "\nMore information about the error:\n\n {:#?}",
                available_updates
            );
        }
    } else {
        eprintln!(
            "File {} doesn't exist, please install the pacman-contrib package.",
            &checkupdates
        );
    }
}