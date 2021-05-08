fn main() {
    let cmd = clap::App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .required(true)
                .value_name("FILE")
                .help("path to the ft-sync config file")
                .takes_value(true),
        )
        .subcommands(vec![
            clap::SubCommand::with_name("status").about("show the sync status"),
            clap::SubCommand::with_name("sync").about("sync files").arg(
                clap::Arg::with_name("dry_run")
                    .help("run in dry run mode")
                    .short("n")
                    .long("dry-run"),
            ),
        ])
        .get_matches();

    let config_file = cmd.value_of("config").unwrap();
    println!("Config File: {}", config_file);

    let _config = match cmd.subcommand() {
        ("status", _) => {
            match ft_sync::commands::status(config_file) {
                Ok(()) => {},
                Err(e) => println!("{:?}", e.to_string())
            }
            println!("Command: status()");
        },
        ("sync", _args) => {
            println!("syncing");
            match ft_sync::commands::sync(config_file, false){
                Ok(()) => {},
                Err(e) => println!("{:?}", e.to_string())
            }
        }
        (_, _) => todo!("impossible!"),
    };
}
