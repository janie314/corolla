use clap::Parser;

mod corolla;

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
    /// Base URL for API endpoints
    #[arg(short, long, default_value_t = String::from(""))]
    route: String,
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
        corolla::serve(
            &args.route,
            &args.db,
            args.port,
            &["create table if not exists t (c text);"],
            &[
                (
                    "q1",
                    "select c from t where c != ?;",
                    Vec::from(["val".to_string()]),
                ),
                (
                    "q2",
                    "insert into t values (?);",
                    Vec::from(["val".to_string()]),
                ),
            ],
        )
        .await
        .expect("the server stopped");
    }
}
