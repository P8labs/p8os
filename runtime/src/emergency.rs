use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

use crate::config::RuntimeConfig;
use crate::core::error::RuntimeError;

pub fn emergency_mode(config: &RuntimeConfig, error: &RuntimeError) -> ! {
    log_error(config, error);

    eprintln!();
    eprintln!("================================");
    eprintln!("P8OS EMERGENCY MODE");
    eprintln!("================================");
    eprintln!("{}", error);
    eprintln!();
    eprintln!("Available actions:");
    eprintln!("  r = reboot");
    eprintln!("  p = poweroff");
    eprintln!("================================");

    loop {
        let mut input = String::new();

        if std::io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        match input.trim() {
            "r" => reboot(),
            "p" => poweroff(),
            _ => {}
        }
    }
}

fn log_error(config: &RuntimeConfig, error: &RuntimeError) {
    let line = format!("[{:?}] {}\n", std::time::SystemTime::now(), error);

    let _ = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config.log_path)
        .and_then(|mut file| file.write_all(line.as_bytes()));
}

fn reboot() -> ! {
    let _ = Command::new("reboot").status();
    loop {
        std::thread::park();
    }
}

fn poweroff() -> ! {
    let _ = Command::new("poweroff").status();
    loop {
        std::thread::park();
    }
}
