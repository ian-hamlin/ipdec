use std::net::{IpAddr};
use std::str::FromStr;
use std::env;
use std::process::exit;

fn main() {
    /*  
        Simple attempt to take in a value, convert it to an IP and output the decimal value.
        
        Sample IP
        2405:0205:8481:84DB:76A9:4824:523E:917C
        216.147.214.045
    */
    // Read the args.  Arg 0 is always the name/path of the command
    let args: Vec<String> = env::args().collect();
    
    // Check if we have our 2nd arg
    if args.len() < 2 {
        println!("At least one argument is required.");
        exit(0)
    }

    // Get the first arg, which should be the IP
    let search_ip = &args[1];

    // Convert the string to an actual IP
    let ip = IpAddr::from_str(search_ip);
    
    // Match on a result and do some work if OK.
    match ip {
        Result::Ok(val) => 
            match val {
                IpAddr::V4(ip_v4) => {
                    let numeric =  u32::from(ip_v4);
                    println!("IPv4 {} {}", ip_v4, numeric);
                },
                IpAddr::V6(ip_v6) => {
                    let numeric =  u128::from(ip_v6);
                    println!("IPv6 {} {}", ip_v6, numeric);
                },
            },               
        Result::Err(err) => println!("BAD IP {:?}", err)
    }
}