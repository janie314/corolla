use clap::Parser;

mod db;
mod endpoints;

/// "your liteweight backend"
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Choose a port to listen on
    #[arg(short, long, default_value_t = String::from("corolla.sqlite3"))]
    db: String,
    /// Choose a port to listen on
    #[arg(short, long, default_value_t = 50000)]
    port: i64,
    /// Test mode?
    #[arg(short, long)]
    test: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.test {
        println!("testing, 123.")
    } else {
        endpoints::serve(&args.db, args.port).await;
    }
}
