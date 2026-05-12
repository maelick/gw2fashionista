# GW2 Fashion Exporter

CLI tool to export GW2 equipment tabs as fashion templates

## Requirements

* Python 3.11+
* [Poetry](https://python-poetry.org/)

## Running the CLI

The CLI can be run using poetry and display help for:
* the CLI including list of subcommands: `poetry run python3 -m gw2fashionista --help`
* a subcommand: `poetry run python3 -m gw2fashionista <subcommand> --help`

It includes the following subcommands:
* `export`: export one or several characters' equipment tabs as fashion templates using 
  It requires an API key with account, builds and characters permissions.
  The key can be provided as a CLI argument or environment variable (which can be placed in a .env file).
  For example, to export all characters to fashion.csv:
```bash
GW2_API_KEY='<your-api-key-here>' poetry run python3 -m gw2fashionista export -o fashion.csv
``` 
* `read`: read one or several fashion template (given as chat link) and print its content,
  resolving skin and dyes names using the GW2 API.
* `filter`: filter one fashion template (given as chat link) by removing undesired slots.
* `merge` combines two fashion templates (given as chat link) by replacing in the first
  one slots that are set in the second one.
  The second template can be filtered using the same filters as for the `filter` command,
  and the command also allows to only merge dyes or skins if desired.

## Examples

```bash
alias gw2fashionista='poetry run python3 -m gw2fashionista'

# Export all character fashion to fashion.csv
# This can take several minutes if you have a lot of characters
# Verbose flag make it possible to follow 
GW2_API_KEY='<your-api-key-here>' gw2fashionista -v export -o fashion.csv

# Output exported fashion
gw2fashionista read < fashion.csv

fashion1='[&<base64-encoded-template>]'
fashion2='[&<base64-encoded-template]]'

# Pretty print individual fashion templates
gw2fashionista read "$fashion1" | jq

# Strip weapons from fashion2
fashion2_noweapons=$(gw2fashionista filter "$fashion2" --no-weapons)

echo $fashion2_noweapons | gw2fashionista read | jq

gw2fashionista filter "$fashion2" --no-weapons | gw2fashionista read | jq

# Combines fashion1 weapons with fashion2
gw2fashionista merge "$fashion1" "$fashion2_noweapons" | gw2fashionista read | jq

# Showcase the different filtering options and outputs how it affects the backpack:

# fashion2 backpack
gw2fashionista merge "$fashion1" "$fashion2_noweapons" | gw2fashionista read | jq '.[].skins.backpack'
# fashion1 backpack
gw2fashionista merge "$fashion1" "$fashion2_noweapons" --no-backpack | gw2fashionista read | jq '.[].skins.backpack'
# fashion1 backpack skin and fashion2 dyes
gw2fashionista merge "$fashion1" "$fashion2_noweapons" --dyes-only | gw2fashionista read | jq '.[].skins.backpack'
# fashion2 backpack skin and fashion1 dyes
gw2fashionista merge "$fashion1" "$fashion2_noweapons" --skin-only | gw2fashionista read | jq '.[].skins.backpack'
```
