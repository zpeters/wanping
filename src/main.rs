use std::env;
use std::process;

use wanping::pinger;

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
        println!("OK: All up")
    } else if all_down {
        println!("ERR: All down")
    } else {
        println!("WARN: Some down")
    }

}



fn help() {
    println!("Usage:");
    println!("\twanping 1.1.1.1");
    println!("\twanping reddit.com");
    println!("\twanping 1.1.1.1 8.8.8.8 reddit.com");
}
