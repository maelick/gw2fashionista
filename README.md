# GW2 Fashion Exporter

CLI tool to export GW2 equipment tabs as fashion templates

## Requirements

* Python 3.11+
* [Poetry](https://python-poetry.org/)

## Running the CLI

The CLI can be run using poetry and display help for:
* the CLI including list of subcommands: `poetry run python3 -m gw2fashion --help`
* a subcommand: `poetry run python3 -m gw2fashion <subcommand> --help`

Currently only the export subcommand is implemented.
It requires an API key with account, builds and characters permissions.
The key can be provided as a CLI argument or environment variable (which can be placed in a .env file).
For example, to export all characters to fashion.csv:
```
GW2_API_KEY='<your-api-key-here>' poetry run python3 -m gw2fashion -o fashion.csv
``` 
