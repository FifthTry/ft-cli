
use clap::ArgGroup;
use std::env;

extern crate clap;
extern crate gitignore;

fn main() {
    use clap::{App, Arg, SubCommand};

    let cmd = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .required(true)
                .value_name("FILE")
                .help("path to the ftsync config file")
                .takes_value(true),
        )
        .subcommands(vec![
            SubCommand::with_name("status").about("show the sync status"),
            SubCommand::with_name("sync").about("sync files").arg(
                Arg::with_name("dry_run")
                    .help("run in dry run mode")
                    .short("n")
                    .long("dry-run"),
            ),
        ])
        .get_matches();

    let config_file = cmd.value_of("config").unwrap();

    match cmd.subcommand() {
        ("status", _) => {}
        ("sync", args) => {}
        (_, _) => todo!("impossible!"),
    }
}
