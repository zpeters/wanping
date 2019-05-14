extern crate fern;
#[macro_use]
extern crate log;
extern crate chrono;


pub mod pinger {
    use std::process::Command;
    use std::str;
    
    fn setup_logger() -> Result<(), fern::InitError> {
        fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%dT%H:%M:%S-06:00]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .apply()?;
        Ok(())
    }

    pub fn ping(ip: &str) -> bool {

        let _ = setup_logger();

        let output = if cfg!(windows) {
            Command::new("cmd")
            .args(&["/C", "ping -n 1 -w 1000", ip])
            .output()
            .expect("Unable to ping")
            .stdout
        } else {
            Command::new("ping")
            .args(&["-c 1", "-t 1", ip])
            .output()
            .expect("Unable to ping")
            .stdout
        };
        
        if cfg!(windows) {
            match str::from_utf8(&output) {
                Ok(result) => {
                    if result.contains("(0% loss)") {
                        debug!("Ping {} success", ip);
                        return true
                    } else {
                        warn!("Ping {} failed", ip);
                        return false 
                    }
                },
                Err(e) => {
                    warn!("Ping {} failed", ip);
                    error!("Error: {}", e);
                    return false
                },
            }
        } else {
            match str::from_utf8(&output) {
                Ok(result) => {
                    if result.contains(" 0.0% packet loss") {
                        debug!("Ping {} success", ip);
                        return true
                    } else {
                        warn!("Ping {} failed", ip);
                        return false
                    }
                },
                Err(e) => {
                    warn!("Ping {} failed", ip);
                    error!("Error: {}", e);
                    return false
                },
            }
        }
    }
}