use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use clap::{value_t, App, Arg};
use log::debug;

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    debug!("ferris_watch starting...");

    let matches = App::new("ferris_watch")
        .version("0.1.0")
        .author("Masaki Hara <ackie.h.gmai@gmail.com>")
        .about("cute watch command")
        .arg(
            Arg::with_name("command")
                .required(true)
                .multiple(true)
                .help("The command to run periodically"),
        )
        .arg(
            Arg::with_name("interval")
                .long("interval")
                .short("n")
                .takes_value(true)
                .default_value("2.0")
                .help("The period to run a command"),
        )
        .get_matches();

    let command = matches.values_of("command").unwrap().collect::<Vec<_>>();
    let interval = value_t!(matches, "interval", f64)?;
    debug!("command = {:?}", command);
    debug!("interval = {:?}", interval);
    let interval10 = (interval * 10.0) as u32;

    let interrupted = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGINT, interrupted.clone())?;
    let interrupted = || interrupted.load(Ordering::SeqCst);

    'outer: loop {
        let output = Command::new(command[0]).args(&command[1..]).output()?;
        debug!("output = {:?}", output);
        let output = String::from_utf8_lossy(&output.stdout);
        println!("{}", output);

        for _ in 0..interval10 {
            sleep(Duration::from_millis(100));
            if interrupted() {
                break 'outer;
            }
        }
        if interrupted() {
            break 'outer;
        }
    }

    log::debug!("end");
    Ok(())
}
