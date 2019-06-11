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
                .args(&["/C", "ping -n 1 -w 4000", ip])
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
                        true
                    } else {
                        warn!("Ping {} failed", ip);
                        false
                    }
                }
                Err(e) => {
                    warn!("Ping {} failed", ip);
                    error!("Error: {}", e);
                    false
                }
            }
        } else {
            match str::from_utf8(&output) {
                Ok(result) => {
                    if result.contains(" 0.0% packet loss") {
                        debug!("Ping {} success", ip);
                        true
                    } else {
                        warn!("Ping {} failed", ip);
                        false
                    }
                }
                Err(e) => {
                    warn!("Ping {} failed", ip);
                    error!("Error: {}", e);
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_good_internal() {
        assert_eq!(pinger::ping("127.0.0.1"), true);
    }
    
    #[test]
    fn test_ping_good_external() {
        assert_eq!(pinger::ping("1.1.1.1"), true);
    }

      #[test]
    fn test_ping_good_internal_dns() {
        assert_eq!(pinger::ping("localhost"), true);
    }

      #[test]
    fn test_ping_good_external_dns() {
        assert_eq!(pinger::ping("google.com"), true);
    }
    
    #[test]
    fn test_ping_bad_ip() {
        assert_eq!(pinger::ping("55.55.55.55"), false);
    }

    #[test]
    fn test_ping_bad_dns() {
        assert_eq!(pinger::ping("this.is.a.fake"), false);
    }

    #[test]
    fn test_ping_bad_garbage() {
        assert_eq!(pinger::ping("fdskafjdsjf dajfdkjk;adsjf;dls"), false);
    }

}