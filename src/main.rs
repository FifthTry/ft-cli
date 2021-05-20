fn main() {
    let cmd = clap::App::new(env!("CARGO_PKG_NAME"))
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
        .subcommand(clap::SubCommand::with_name("sync").about("sync files"))
        .get_matches();

    let config = ft_sync::Config::from_file("ft-sync.p1").expect("failed to read config");

    match cmd.subcommand() {
        ("status", _) => match ft_sync::status(&config) {
            Ok(()) => {}
            Err(e) => println!("{}", e.to_string()),
        },
        ("sync", _args) => match ft_sync::sync(&config) {
            Ok(()) => {}
            Err(e) => println!("{}", e.to_string()),
        },
        (_, _) => todo!("impossible!"),
    };
}
