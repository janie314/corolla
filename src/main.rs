use clap::Parser;
use log::{error, info, LevelFilter};
use std::{env, fs::File, io::Write, process};

mod corolla;

/// "your liteweight backend"
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filepath to the SQLite database
    #[arg(short, long, default_value_t = String::from("corolla.sqlite3"))]
    db: String,
    /// Write out the server process's PID to this file
    #[arg(long)]
    pid_file: Option<String>,
    /// Choose a port to listen on
    #[arg(short, long, default_value_t = 50000)]
    port: i64,
    /// Base URL for API endpoints
    #[arg(short, long, default_value_t = String::from(""))]
    route: String,
    /// Filepath to the spec.json file
    #[arg(short, long, default_value_t = String::from("spec.json"))]
    spec: String,
    /// Filepath to static file directory
    #[arg(long, default_value_t = String::from("public"))]
    r#static: String,
    /// Test mode?
    #[arg(short, long)]
    test: bool,
}

#[tokio::main]
async fn main() {
    // parse CLI args
    let args = Args::parse();
    // init logging; default logging level is INFO
    match env::var("RUST_LOG") {
        Ok(_) => pretty_env_logger::init(),
        Err(_) => pretty_env_logger::formatted_builder()
            .filter(None, LevelFilter::Info)
            .init(),
    }
    info!("corolla v{}", env!("CARGO_PKG_VERSION"));
    if args.test {
    } else {
        match args.pid_file {
            Some(path) => {
                let mut file = File::create(&path).expect(&format!("could not open {}", path));
                file.write_all(process::id().to_string().as_bytes())
                    .expect(&format!("could not write PID to {}", path));
            }
            None => (),
        };
        let res = corolla::run(&args.route, args.port, &args.db, &args.r#static, &args.spec).await;
        match res {
            Ok(_) => (),
            Err(e) => {
                error!("{:?}", e);
                process::exit(1)
            }
        }
    }
}
