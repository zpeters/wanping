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
    let mut results = Vec::new();

    for e in addrs {
        results.push(pinger::ping(e))
    }

    let all_up = results.iter().all(|&x| x == true);
    let all_down = results.iter().all(|&x| x == false);

    if all_up {
        println!("ALLUP: All ips are up")
    } else if all_down {
        println!("ALLDOWN: All ips are down")
    } else {
        println!("SOMEDOWN: Some ips are down")
    }

}

fn help() {
    println!("wanping - {} - Zach Peters", VERSION);
    println!("Usage:");
    println!("\twanping 1.1.1.1");
    println!("\twanping reddit.com");
    println!("\twanping 1.1.1.1 8.8.8.8 reddit.com");
}
