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
            clap::SubCommand::with_name("sync")
                .about("sync files")
                .subcommand(
                    clap::SubCommand::with_name("all")
                        .about("re-sync all document")
                        .help("re-sync all document"),
                ),
        )
        .get_matches();

    let config = ft_cli::Config::from_file("ft-sync.p1").expect("failed to read config");

    match cmd.subcommand() {
        ("status", _) => match ft_cli::status(&config) {
            Ok(()) => {}
            Err(e) => println!("{}", e.to_string()),
        },
        ("sync", args) => {
            let re_sync = if let Some(args) = args {
                match args.subcommand() {
                    ("all", _) => true,
                    (t, _) => {
                        eprintln!("unknown subcommand: {}", t);
                        false
                    }
                }
            } else {
                false
            };
            match ft_cli::sync(&config, re_sync) {
                Ok(()) => {
                    println!("{:?}", args);
                }
                Err(e) => println!("{}", e.to_string()),
            }
        }
        (_, _) => todo!("impossible!"),
    };
}
