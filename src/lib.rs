pub mod pinger {
    use std::process::Command;
    use std::str;

    pub fn ping(ip: &str) -> bool {
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
                        println!("Ping {} success", ip);
                        return true
                    } else {
                        println!("Ping {} failed", ip);
                        return false
                   }
                },
                Err(_) => {
                    println!("Ping {} failed", ip);
                    return false
                },
            }
        } else {
            match str::from_utf8(&output) {
                Ok(result) => {
                    if result.contains(" 0.0% packet loss") {
                        println!("Ping {} success", ip);
                        return true
                    } else {
                        println!("Ping {} failed", ip);
                        return false
                    }
                },
                Err(_) => {
                    println!("Ping {} failed", ip);
                    return false
                },
            }
        }
    }
}