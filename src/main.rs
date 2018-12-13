use clap::{value_t, App, Arg};

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    log::debug!("ferris_watch starting...");

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
    eprintln!("{:?}", command);
    eprintln!("{:?}", interval);

    Ok(())
}
