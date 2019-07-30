extern crate clap;

use clap::{App, AppSettings, Arg};

use wanping::pinger;

fn main() {
    let matches = App::new("Wan Ping")
        .version("0.1.3")
        .author("Zach Peters")
        .about("Ping multiple ip addresses")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("timeout")
                .short("t")
                .long("timeout")
                .value_name("TIMEOUT")
                .default_value("4000")
                .help("Sets the ping timeout")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("servers")
                .value_name("SERVERS_TO_PING")
                .help("Servers to ping")
                .index(1)
                .required(true)
                .multiple(true),
        )
        .get_matches();

    if matches.is_present("servers") {
        let servers = matches.values_of("servers").unwrap().collect::<Vec<_>>();
        let timeout = matches.value_of("timeout").unwrap();
        let addrs = &servers[0..];
        let mut pings = Vec::new();

        for e in addrs {
            pings.push(pinger::ping(e, timeout))
        }

        println!("{}", results(pings))
    }
}

fn results(pings: Vec<bool>) -> String {
    let all_up = pings.iter().all(|&x| x);
    let all_down = pings.iter().all(|&x| !x);

    if all_up {
        "ALLUP: All ips are up".to_string()
    } else if all_down {
        "ALLDOWN: All ips are down".to_string()
    } else {
        "SOMEDOWN: Some ips are down".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_up_single() {
        let mut pings = Vec::new();
        pings.push(true);
        assert_eq!(results(pings), "ALLUP: All ips are up");
    }

    #[test]
    fn test_all_up_multiple() {
        let mut pings = Vec::new();
        pings.push(true);
        pings.push(true);
        pings.push(true);
        pings.push(true);
        assert_eq!(results(pings), "ALLUP: All ips are up");
    }

    #[test]
    fn test_some_down() {
        let mut pings = Vec::new();
        pings.push(true);
        pings.push(false);
        assert_eq!(results(pings), "SOMEDOWN: Some ips are down");
    }

    #[test]
    fn test_some_down_multiple() {
        let mut pings = Vec::new();
        pings.push(true);
        pings.push(false);
        pings.push(false);
        pings.push(true);
        pings.push(false);
        pings.push(true);
        pings.push(true);
        pings.push(false);
        pings.push(false);
        pings.push(true);
        pings.push(false);
        pings.push(true);
        pings.push(true);
        pings.push(false);
        pings.push(false);
        pings.push(true);
        pings.push(false);
        pings.push(true);
        assert_eq!(results(pings), "SOMEDOWN: Some ips are down");
    }
    #[test]
    fn test_all_down_single() {
        let mut pings = Vec::new();
        pings.push(false);
        assert_eq!(results(pings), "ALLDOWN: All ips are down");
    }

    #[test]
    fn test_all_down_multiple() {
        let mut pings = Vec::new();
        pings.push(false);
        pings.push(false);
        pings.push(false);
        pings.push(false);
        assert_eq!(results(pings), "ALLDOWN: All ips are down");
    }

}
