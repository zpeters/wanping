pub mod pinger {
    use std::process::Command;
    use std::str;

    pub fn ping(ip: &str, timeout: &str) -> bool {
        let output = if cfg!(windows) {
            Command::new("cmd")
                .args(&["/C", "ping -n 1 -w", timeout, ip])
                .output()
                .expect("Unable to ping")
                .stdout
        } else {
            Command::new("ping")
                .args(&["-c 1", "-t", timeout, ip])
                .output()
                .expect("Unable to ping")
                .stdout
        };

        if cfg!(windows) {
            match str::from_utf8(&output) {
                Ok(result) => result.contains("(0% loss)"),
                Err(e) => {
                    println!("Error: {:?}", e);
                    false
                }
            }
        } else {
            match str::from_utf8(&output) {
                Ok(result) => {
                    result.contains(" 0.0% packet loss") || result.contains(" 0% packet loss")
                }
                Err(e) => {
                    println!("Error: {:?}", e);
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
        let timeout = "100";
        assert_eq!(pinger::ping("127.0.0.1", timeout), true);
    }

    #[test]
    fn test_ping_good_external() {
        let timeout = "100";

        assert_eq!(pinger::ping("8.8.8.8", timeout), true);
    }

    #[test]
    fn test_ping_good_internal_dns() {
        let timeout = "100";

        assert_eq!(pinger::ping("localhost", timeout), true);
    }

    #[test]
    fn test_ping_good_external_dns() {
        let timeout = "100";

        assert_eq!(pinger::ping("google.com", timeout), true);
    }

    #[test]
    fn test_ping_bad_ip() {
        let timeout = "100";

        assert_eq!(pinger::ping("55.55.55.55", timeout), false);
    }

    #[test]
    fn test_ping_bad_dns() {
        let timeout = "100";

        assert_eq!(pinger::ping("this.is.a.fake", timeout), false);
    }

    #[test]
    fn test_ping_bad_garbage() {
        let timeout = "100";

        assert_eq!(
            pinger::ping("fdskafjdsjf dajfdkjk;adsjf;dls", timeout),
            false
        );
    }
}
