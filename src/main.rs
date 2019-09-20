extern crate clap;
extern crate psutil;
use clap::{App, Arg, ArgMatches};
use psutil::memory::*;
use std::process;

fn main() {
    let cli_matches = App::new("check_mem")
        .author("lagooj")
        .version("0.1.0")
        .about("Check memory or swap for icinga / nagios")
        .arg(
            Arg::with_name("TYPE")
                .help("type mem or swap")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("WARNING")
                .short("w")
                .long("warning")
                .help("Set warning threshold")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("CRITICAL")
                .short("c")
                .long("critical")
                .help("Set critical threshold")
                .takes_value(true)
                .required(true),
        )
        .get_matches();


    println!("{:?}", cli_matches);
    let warning = get_thresholds(&cli_matches, "WARNING").unwrap();
    let critical = get_thresholds(&cli_matches, "CRITICAL").unwrap();

    if let Some(t) = cli_matches.value_of("TYPE") {
        if t == "men" {
            let vm = virtual_memory().unwrap();
            match monitoring_out("Memory", vm.percent, warning, critical) {
                Ok(ret_code) => process::exit(ret_code),
                Err(err_code) => process::exit(err_code),
            };
        } else if t == "swap" {
            let swap = swap_memory().unwrap();
            match monitoring_out("Swap", swap.percent, warning, critical) {
                Ok(ret_code) => process::exit(ret_code),
                Err(err_code) => process::exit(err_code),
            };
        } else {
            println!("UNKNOWN: Type {} Unknown", t);
            process::exit(3)
        }
    }
}

fn get_thresholds(clap_args: &ArgMatches, threshold_type: &str) -> Result<f32, i8> {
    if let Some(w) = clap_args.value_of(threshold_type) {
        return Ok(w.parse::<f32>().expect("Should return float"));
    }
    Err(-1)
}

fn monitoring_out(probe_type: &str, percent: f32, warn: f32, crit: f32) -> Result<i32, i32> {
    if warn < 0.0 || crit < 0.0 {
        panic!("Threshold should not be negative");
    }
    if percent > crit {
        println!("Critical: {} {}% - {}%", probe_type, percent, crit);
        Ok(2)
    } else if percent > warn {
        println!("Warning: {} {}% - {}%", probe_type, percent, warn);
        Ok(1)
    } else if percent < crit && percent < warn {
        println!("OK: {} {}%", probe_type, percent);
        Ok(0)
    } else {
        println!("UNKNOWN: {} Error", probe_type);
        Err(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_monitoring_out_swap_ok() {
        assert_eq!(monitoring_out("swap", 80.0, 81.0, 82.0), Ok(0));
    }
    #[test]
    fn test_monitoring_out_swap_warning() {
        assert_eq!(monitoring_out("swap", 81.0, 80.0, 82.0), Ok(1));
    }
    #[test]
    fn test_monitoring_out_swap_critical() {
        assert_eq!(monitoring_out("swap", 83.0, 80.0, 82.0), Ok(2));
    }
    #[test]
    #[should_panic]
    fn test_monitoring_out_swap_panic() {
        assert_eq!(monitoring_out("swap", -83.0, 80.0, 82.0), Ok(3));
    }
    #[test]
    #[should_panic]
    fn test_monitoring_out_swap_panic2() {
        assert_eq!(monitoring_out("swap", 83.0, 80.0, -82.0), Ok(3));
    }
    #[test]
    #[should_panic]
    fn test_monitoring_out_swap_panic3() {
        assert_eq!(monitoring_out("swap", 83.0, -80.0, 85.0), Ok(3));
    }

}
