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
    //let mut results = [];

    // let results = for a in addrs.iter() {
    //     pinger::ping(a);
    // };

    // this is driving me nuts
    //http://xion.io/post/code/rust-iter-patterns.html

    let results: &[bool] = addrs
        .iter()
        .filter_map(|x| pinger::ping(x) == true)
        .collect();


    println!("Res: {:?}", results);
    

    //println!("print results");
    
    // do a ping
    //let addr =  &args[1];
    //let res = pinger::ping(addr);
    //println!("Ping result: {:#?}", res);
}



fn help() {
    println!("Usage:");
    println!("\twanping 1.1.1.1");
    println!("\twanping reddit.com");
    println!("\twanping 1.1.1.1 8.8.8.8 reddit.com");
}
