fn main() {
    let cmd = clap::App::new("ft")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::with_name("test")
                .short("t")
                .long("test")
                .required(false)
                .value_name("TEST")
                .help("if to run in test mode")
                .hidden(true)
                .takes_value(false),
        )
        .subcommand(clap::SubCommand::with_name("status").about("show the sync status"))
        .subcommand(
            clap::SubCommand::with_name("sync").about("sync files").arg(
                clap::Arg::with_name("all")
                    .long("all")
                    .short("a")
                    .allow_hyphen_values(true)
                    .help("re-sync all document"),
            ),
        )
        .subcommand(
            clap::SubCommand::with_name("import")
                .about("import book")
                .arg(
                    clap::Arg::with_name("repo")
                        .long("repo")
                        .allow_hyphen_values(true)
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    clap::Arg::with_name("collection")
                        .long("collection")
                        .allow_hyphen_values(true)
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    clap::Arg::with_name("root")
                        .long("root")
                        .allow_hyphen_values(true)
                        .takes_value(true)
                        .required(false)
                        .default_value(""),
                )
                .arg(
                    clap::Arg::with_name("backend")
                        .long("backend")
                        .allow_hyphen_values(true)
                        .takes_value(true)
                        .required(false)
                        .default_value(""),
                ),
        )
        .get_matches();

    match cmd.subcommand() {
        ("status", _) => {
            let config = ft_cli::Config::from_file("ft-sync.p1").expect("failed to read config");
            match ft_cli::status(&config) {
                Ok(()) => {}
                Err(e) => println!("{}", e.to_string()),
            }
        }
        ("sync", Some(args)) => {
            let config = ft_cli::Config::from_file("ft-sync.p1").expect("failed to read config");
            let re_sync = args.args.get("all").is_some();
            match ft_cli::sync(&config, re_sync) {
                Ok(()) => {}
                Err(e) => println!("{}", e.to_string()),
            }
        }

        ("import", Some(args)) => {
            let config = ft_cli::Config::from_args(args);

            match ft_cli::import(&config, true) {
                Ok(()) => {}
                Err(e) => println!("{}", e.to_string()),
            }
        }
        (_, _) => todo!("impossible!"),
    };
}
