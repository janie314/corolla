# corolla

a lightweight sqlite web server.

# dependencies

- Perl
  - [Carton](https://metacpan.org/pod/Carton)

# installation

```shell
carton install
./corolla
```

# usage

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
