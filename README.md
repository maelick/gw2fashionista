# GW2 Fashionista [![CI](https://github.com/maelick/gw2fashionista/actions/workflows/ci.yaml/badge.svg)](https://github.com/maelick/gw2fashionista/actions/workflows/ci.yaml)

CLI tool to manage GW2 fashion templates.
Currently, it can:
* parse, filter, and merge wardrobe and travel template chat links.
* export equipment tabs as wardrobe templates using the GW2 API.

## Running the CLI

The CLI can be run by
* downloading the released binary and
  * on Linux:
    * renaming it `gw2fashionista`
    * making it executable: `chmod +x gw2fashionista`
  * on Windows: renaming it `gw2fashionista.exe`
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
cargo run wardrobe export -o fashion.csv
``` 
* `wardrobe filter` and `travel filter`: filter a wardrobe or travel template (given as a chat link) by removing undesired slots.
* `wardrobe merge` and `travel merge` combine two wardrobe or travel templates (given as chat links) by replacing in the first
  one slots that are set in the second one.
  The second template can be filtered using the same filters as for the `filter` command,
  and the command also allows merging only dyes or skins, if desired.

## Examples

The following examples are written for Linux and/or bash terminals.

```bash
# Export all character fashion to fashion.csv
GW2_API_KEY='<your-api-key-here>' ./gw2fashionista wardrobe export -o fashion.csv

# Output exported fashion
./gw2fashionista read < fashion.csv

fashion1='[&<base64-encoded-template>]'
fashion2='[&<base64-encoded-template]]'

# Pretty print individual fashion templates
./gw2fashionista read "$fashion1" | jq

# Strip weapons from fashion2
fashion2_noweapons=$(./gw2fashionista wardrobe filter "$fashion2" --exclude weapons)
echo $fashion2_noweapons | ./gw2fashionista read | jq

# Combines fashion1 weapons with fashion2
./gw2fashionista wardrobe merge "$fashion1" "$fashion2_noweapons" | ./gw2fashionista read | jq

# Showcase the different filtering options and how it affects the backpack:

# fashion2 backpack
./gw2fashionista wardrobe merge "$fashion1" "$fashion2_noweapons" | ./gw2fashionista read | jq .backpack
# fashion1 backpack
./gw2fashionista wardrobe merge "$fashion1" "$fashion2_noweapons" --exclude backpack | ./gw2fashionista read | jq .backpack
# fashion1 backpack skin and fashion2 dyes
./gw2fashionista wardrobe merge "$fashion1" "$fashion2_noweapons" --no-skins | ./gw2fashionista read | jq .backpack
# fashion2 backpack skin and fashion1 dyes
./gw2fashionista wardrobe merge "$fashion1" "$fashion2_noweapons" --no-dyes | ./gw2fashionista read | jq .backpack
```
