use std::env;
use std::process;

use wanping::pinger;

const VERSION: &str = "v0.0.1";

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        help();
        process::exit(0);
    }

    let addrs = &args[1..];
    let mut pings = Vec::new();

    for e in addrs {
        pings.push(pinger::ping(e))
    }

    println!("{}", results(pings))
   
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

fn help() {
    println!("wanping - {} - Zach Peters", VERSION);
    println!("Usage:");
    println!("\twanping 1.1.1.1");
    println!("\twanping reddit.com");
    println!("\twanping 1.1.1.1 8.8.8.8 reddit.com");
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