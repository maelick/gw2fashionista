# GW2 Fashionista [![CI](https://github.com/maelick/gw2fashionista/actions/workflows/ci.yaml/badge.svg)](https://github.com/maelick/gw2fashionista/actions/workflows/ci.yaml)

Command Line Interface (CLI) tool to manage GW2 fashion templates.
Currently, it can:
* parse, filter, and merge wardrobe and travel template chat links.
* export equipment tabs as wardrobe templates using the GW2 API.
  (Requires an API key with the `account`, `builds`, and `characters` permissions.)
* store and manage fashion templates and annotate them with a name, character name, description, and tags.

See [CLI.md](./docs/CLI.md) for more information and examples on how to use the CLI.

## Running the CLI

The CLI can be run by
* downloading and executing the released binary
* downloading the sources and
  * using `cargo run`
  * building the binary: `cargo build --release`

It includes the following subcommands:
* `help` and `help <subcommand>` to get the general help or help on a subcommand
* `read`: read one or several GW chat links (currently only supports wardrobe & travel templates) and print their content,
  resolving skin and dye names using the GW2 API.
* `wardrobe export`: export from the GW2 API one or several characters' equipment tabs as fashion templates.
  It requires an API key with the `account`, `builds`, and `characters` permissions.
  The key can be provided as a CLI argument or as an environment variable.
  For example, to export all characters to fashion.csv:
```bash
export GW2_API_KEY='<your-api-key-here>'
./gw2fashionista wardrobe export -o fashion.csv
``` 
* `wardrobe filter` and `travel filter`: filter a wardrobe or travel template (given as a chat link) by removing undesired slots.
* `wardrobe merge` and `travel merge` combine two wardrobe or travel templates (given as chat links) by replacing in the first
  one slots that are set in the second one.
  The second template can be filtered using the same filters as for the `filter` command,
  and the command also allows merging only dyes or skins, if desired.
* `fashion create|get|set|patch|delete|list|untag`: manage stored fashion templates
* `fashion tag list|clean`: manage tags
* `fashion wardrobe|travel get|set`: get or set the chat link of the wardrobe or travel template of a stored fashion template

## Development

This project uses [just](https://github.com/casey/just) to easily build, lint, and run this project.
See the [justfile](./justfile) or run `just` in the root of the repository for the list of available targets.

### Running the tests

```bash
cargo test --all-features
```

The e2e tests for the wardrobe export call the real GW2 API and require a `GW2_API_KEY`
environment variable (same permissions as `wardrobe export`, see above).
Without it, those tests fail.

The `storage` crate uses [sqlx](https://github.com/launchbadge/sqlx) with compile-time
checked queries (`sqlx::query!`/`query_as!`).
These macros need to validate each query against a real schema at compile time, using one of two modes:
* **online**: set `DATABASE_URL` to a SQLite database with the crate's migrations
  applied, e.g. `DATABASE_URL=sqlite://path/to/db.sqlite`.
  You can setup the database with the sqlx-cli (see below).
* **offline**: set `SQLX_OFFLINE=true`. This reads the query metadata cache committed
  in the workspace-root `.sqlx` directory instead of connecting to a database
  This is what CI uses, and what you'll want if you don't have a database set up locally.

If a query in the `storage` crate is added or changed, the cache (requires
`sqlx-cli` and a real database, i.e. online mode) needs to be regenerated and the result committed:

```bash
sqlx database drop
sqlx database create
sqlx migrate run --source storage/migrations
cargo sqlx prepare --workspace -- --all-targets
git add .sqlx
git commit -m "chore: prepare sqlx queries"
```
