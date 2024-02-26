# Corolla

Corolla is a a lightweight SQLite web server. You can use it as a bare-bones web
framework.

# Example Application

This Git repostiory includes example Corolla
[specs](https://github.com/janie314/corolla/blob/main/examples/), which are JSON
files that define a Corolla server's available queries, DB schema, and
conversions.

To run the example Corolla spec:

1. Install [Rust](https://www.rust-lang.org/).

2. Clone this Git repository and navigate into it:

```bash
git clone https://github.com/janie314/corolla
cd corolla
```

3. Start the Corolla server:

```bash
cargo run -- -s examples/example_spec.json
```

Now you can make write queries to the database:

4.

```bash
# curl -s -X POST http://localhost:50000/write/write01 \
  -H 'content-type: application/json' \
  -d '{ "a": "sandringham" }'
```

... And read queries!

5.

```bash
# curl -s http://localhost:50000/read/read01 | jq
[
  [
    "c"
  ],
  [
    "sandringham"
  ]
]
```

# Usage

```
Usage: corolla [OPTIONS]

Options:
  -d, --db <DB>        Filepath to the SQLite database [default: corolla.sqlite3]
  -p, --port <PORT>    Choose a port to listen on [default: 50000]
  -r, --route <ROUTE>  Base URL for API endpoints [default: ]
  -s, --spec <SPEC>    Filepath to the spec.json file [default: spec.json]
  -t, --test           Test mode?
  -h, --help           Print help
  -V, --version        Print version
```
