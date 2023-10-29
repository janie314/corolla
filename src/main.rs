use clap::Parser;

mod corolla;

/// "your liteweight backend"
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filepath to the SQLite database
    #[arg(short, long, default_value_t = String::from("corolla.sqlite3"))]
    db: String,
    /// Choose a port to listen on
    #[arg(short, long, default_value_t = 50000)]
    port: i64,
    /// Base URL for API endpoints
    #[arg(short, long, default_value_t = String::from(""))]
    route: String,
    /// Filepath to the spec.json file
    #[arg(short, long, default_value_t = String::from("spec.json"))]
    spec: String,
    /// Test mode?
    #[arg(short, long)]
    test: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.test {
    } else {
        corolla::run(&args.route, args.port, &args.db, &args.spec)
            .await
            .expect("the server stopped");
    }
}
