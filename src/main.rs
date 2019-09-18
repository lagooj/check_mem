extern crate psutil;
extern crate clap;
use psutil::memory::*;
use std::process;

fn main() {
    let vm = virtual_memory().unwrap();
//    let swap = swap_memory().unwrap();
    match monitoring_out("Memory", vm.percent, 1.00 ,10.00) {
        Ok(ret_code) => process::exit(ret_code),
        Err(err_code) => process::exit(err_code),
    };
/*    match monitoring_out("Swap", swap.percent, 1.00 ,10.00) {
        Ok(ret_code) => process::exit(ret_code),
        Err(err_code) => process::exit(err_code),
    };*/
}

fn monitoring_out(probe_type :&str, percent: f32, warn: f32, crit: f32) -> Result<i32, i32> {
    if percent > crit {
        println!("Critical: {} {}% - {}%", probe_type, percent, crit);
        Ok(2)
    } else if percent > warn {
        println!("Warning: {} {}% - {}%", probe_type, percent, warn);
        Ok(1)
    } else if percent < crit && percent < warn{
        println!("OK: {} {}%",probe_type, percent);
        Ok(0)
    } else {
        println!("UNKNOWN: {} Error",probe_type);
        Err(3)
    }
}
