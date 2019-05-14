pub mod pinger {
    use std::process::Command;
    use std::str;

    pub fn ping(ip: &str) -> bool {
        println!("Ping from lib {}", ip);

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
                        return true
                    } else {
                        return false
                   }
                },
                Err(_) => return false,
            }
        } else {
            match str::from_utf8(&output) {
                Ok(result) => {
                    if result.contains(" 0.0% packet loss") {
                        return true
                    } else {
                        return false
                    }
                },
                Err(_) => return false,
            }
        }
    }
}