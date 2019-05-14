pub mod pinger {
    use std::error::Error;
    use std::process::Command;

    pub fn ping(ip: &str) -> Result<(), Box<Error>> {
        println!("Ping from lib {}", ip);
        let output = Command::new("cmd")
            .args(&["/C", "ping -n 1", ip])
            .output()?;

        // String::from_utf8(output.stdout)?
        //     .lines()
        //     .filter(|s| s.contains("(0% loss)"))
        //     .for_each(|x| println!("{:?}", x));
        
        let res = String::from_utf8(output.stdout)?
            .lines()
            .filter(|s| s.contains("(0% loss)"));

        println!("Result: {:#?}", res);

        Ok(())
    }
}