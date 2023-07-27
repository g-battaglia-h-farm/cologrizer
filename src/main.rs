use std::io::{self, BufRead};

const RED_COLOR: &str = "\x1b[1;31m";
const YELLOW_COLOR: &str = "\x1b[1;33m";
const GREEN_COLOR: &str = "\x1b[1;32m";
const BLUE_COLOR: &str = "\x1b[1;34m";
const MAGENTA_COLOR: &str = "\x1b[1;35m";


fn colorize_log(log_line: &str) -> String {
    if log_line.contains("EMERGENCY") {
        return format!("{}{}", RED_COLOR, log_line);
    } else if log_line.contains("ALERT") {
        return format!("{}{}", RED_COLOR, log_line);
    } else if log_line.contains("CRITICAL") {
        return format!("{}{}", RED_COLOR, log_line);
    } else if log_line.contains("ERROR") {
        return format!("{}{}", RED_COLOR, log_line);
    } else if log_line.contains("WARNING") {
        return format!("{}{}", YELLOW_COLOR, log_line);
    } else if log_line.contains("NOTICE") {
        return format!("{}{}", BLUE_COLOR, log_line);
    } else if log_line.contains("INFO") {
        return format!("{}{}", GREEN_COLOR, log_line);
    } else if log_line.contains("DEBUG") {
        return format!("{}{}", MAGENTA_COLOR, log_line);
    } else {
        return format!("{}", log_line);
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(log_line) = line {
            let colored_log = colorize_log(&log_line);
            println!("{}", colored_log);
        }
    }
}
