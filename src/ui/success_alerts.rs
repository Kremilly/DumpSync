extern crate colored;

use colored::*;

use crate::utils::date::Date;

pub struct SuccessAlerts;

impl SuccessAlerts {

    pub fn dump(file: &str) {
        let current_datetime = Date::date_time();
    
        print!(
            "\r{} Dump successfully completed and saved at {}", 
            current_datetime.green().bold(), 
            file.blue()
        );
    }

    pub fn terminate() {
        let current_datetime = Date::date_time();

        println!(
            "\n\n{} {}",
            current_datetime.green().bold(),
            "Process terminated by user. Exiting gracefully...".red().bold(),
        );
    }

}
