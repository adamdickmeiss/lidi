use clap::{Arg, Command};
use diode::file;
use std::{env, net, path, str::FromStr};

fn main() {
    let args = Command::new(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("from_tcp")
                .long("from_tcp")
                .value_name("ip:port")
                .default_value("127.0.0.1:7000")
                .help("Address and port to listen for diode-receive"),
        )
        .arg(
            Arg::new("from_unix")
                .long("from_unix")
                .value_name("path")
                .help("Patch to listen for diode-receive"),
        )
        .arg(
            Arg::new("buffer_size")
                .long("buffer_size")
                .value_name("nb_bytes")
                .default_value("4194304") // 4096 * 1024
                .value_parser(clap::value_parser!(usize))
                .help("Size of TCP write buffer"),
        )
        .arg(
            Arg::new("output_directory")
                .value_name("dir")
                .default_value(".")
                .help("Output directory"),
        )
        .get_matches();

    let from_tcp = args
        .get_one::<String>("from_tcp")
        .map(|s| net::SocketAddr::from_str(s).expect("invalid from_tcp parameter"));
    let from_unix = args
        .get_one::<String>("from_unix")
        .map(|s| path::PathBuf::from_str(s).expect("from_unix must point to a valid path"));
    let buffer_size = *args.get_one::<usize>("buffer_size").expect("default");
    let output_directory =
        path::PathBuf::from(args.get_one::<String>("output_directory").expect("default"));

    let diode = file::DiodeReceive {
        from_tcp,
        from_unix,
    };

    let config = file::Config { diode, buffer_size };

    init_logger();

    if let Err(e) = file::receive::receive_files(&config, &output_directory) {
        log::error!("{e}");
    }
}

fn init_logger() {
    if env::var("RUST_LOG").is_ok() {
        simple_logger::init_with_env()
    } else {
        simple_logger::init_with_level(log::Level::Info)
    }
    .expect("logger initialization")
}
