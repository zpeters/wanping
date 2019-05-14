use std::env;
use std::process;

use wanping::pinger;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        help();
        process::exit(0);
    }

    println!("You gave me '{:?}'", args);
    //println!("parse input");
    //println!("print results");
    let addr =  &args[1];
    let res = pinger::ping(addr);
    println!("Ping result: {:#?}", res);
}



fn help() {
    println!("This is help");
}
