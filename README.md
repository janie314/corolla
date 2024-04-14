# Corolla

Corolla is a a lightweight SQLite web server. You can use it as a bare-bones web
framework.

# Example

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
  -d '{ "vacation_spot": "sandringham" }'
```

... And read queries!

5.

```bash
# curl -s http://localhost:50000/read/read01 | jq
[
  [
    "vacation_spot"
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

# JavaScript API

Available via npm at `corolla_api`.

See the
[js_api folder and README](https://github.com/janie314/corolla/tree/main/examples)
for more information.

# Development

Issues and pull requests welcome!

## Testing

Uses Rust's testing framework and [Bun's](https://bun.sh/) test command.

To test the Corolla server:

```bash
cargo test
```

To test the JS/TS API:

```bash
bun test
```

## How to Take a Dependabot Update

1. Rebase against dependabot branch.
1. `cargo update`
1. Run tests.
1. Accept merge request.
1. git tag current version, then `git push --tags`.
1. `cargo publish`
