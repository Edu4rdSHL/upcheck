// Crate for notifications
extern crate notify_rust;
extern crate schedule_recv;

use notify_rust::Notification;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let execution_time: u32 = args[1]
            .trim()
            .parse()
            .expect("Error, execution time argument can not be converted in a valid number.");

        println!(
            "You are going to check updates every {} minute(s).",
            execution_time
        );

        // Execution time need to be miliseconds
        let execution_time = execution_time * 60000;
        let tick = schedule_recv::periodic_ms(execution_time);
        loop {
            tick.recv().unwrap();
            upcheck();
        }
    } else {
        println!(
            "
    Usage:

    upcheck <time in minutes>
    Example:

    upcheck 60
    - Check for updates every 60 minutes.
                 "
        );
    }
}

fn send_notification(summary: &str, data: &str, icon: &str) {
    Notification::new()
        .summary(&summary)
        .body(&data)
        .icon(&icon)
        .show()
        .unwrap();
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
            if available_updates.is_empty() {
                send_notification(
                    "No updates are available.",
                    &available_updates,
                    "dialog-information",
                );
            } else {
                send_notification(
                    "The following updates are available:",
                    &available_updates,
                    "dialog-information",
                );
            }
        } else {
            eprintln!("Process exited with: {}", &available_updates.status);
            eprintln!(
                "\nMore information about the error:\n\n {:#?}",
                &available_updates
            );
        }
    } else {
        eprintln!(
            "File {} doesn't exist, please install the pacman-contrib package.",
            &checkupdates
        );
    }
}
