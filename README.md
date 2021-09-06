# Plotone di Esecuzione

[![license](https://img.shields.io/github/license/The-Data-Appeal-Company/plotone-di-esecuzione.svg)](LICENSE)

![alt Francisco Goya, Il 3 maggio 1808 (1814)](https://upload.wikimedia.org/wikipedia/commons/4/48/El_Tres_de_Mayo%2C_by_Francisco_de_Goya%2C_from_Prado_in_Google_Earth.jpg)
Francisco Goya, Il 3 maggio 1808 (1814)

Plotone-di-Esecuzione is a tool to make concurrent queries on postgres read from file.

## Build
```
cargo build --release
```

## Usage
```
Usage: plotone-di-esecuzione [-P <port>] -h <host> -n <name> -u <user> -p <password> [-c <concurrency>] -i <input>

Execute parallel query on postgres.

Options:
-P, --port        db port. default: 5432
-h, --host        db host
-n, --name        db name
-u, --user        db username
-p, --password    db password
-c, --concurrency number of parallel queries. default: 5
-i, --input       queries file path
--help            display usage information
```
