# GW2 Fashionista

CLI tool to export manage GW2 fashion templates

## Running the CLI

The CLI can be run using cargo and display help for:
* the CLI including list of subcommands: `cargo run help`
* a subcommand: `cargo run help <subcommand>`

It includes the following subcommands:
* `read`: read one or several GW chat links (currently only support wardrobe templates) and print its content,
  resolving skin and dyes names using the GW2 API.
* `wardrobe export`: export from the GW2 API one or several characters' equipment tabs as fashion templates.
  It requires an API key with account, builds and characters permissions.
  The key can be provided as a CLI argument or environment variable (which can be placed in a .env file).
  For example, to export all characters to fashion.csv:
```bash
GW2_API_KEY='<your-api-key-here>' cargo run wardrobe export -o fashion.csv
``` 
* `wardrobe filter`: filter one wardrobe template (given as chat link) by removing undesired slots.
* `wardrobe merge` combines two wardrobe templates (given as chat link) by replacing in the first
  one slots that are set in the second one.
  The second template can be filtered using the same filters as for the `filter` command,
  and the command also allows to only merge dyes or skins if desired.

## Examples

```bash
alias gw2fashionista='cargo run'

# Export all character fashion to fashion.csv
# This can take several minutes if you have a lot of characters
GW2_API_KEY='<your-api-key-here>' gw2fashionista wardrobe export -o fashion.csv

# Output exported fashion
gw2fashionista read < fashion.csv

fashion1='[&<base64-encoded-template>]'
fashion2='[&<base64-encoded-template]]'

# Pretty print individual fashion templates
gw2fashionista read "$fashion1" | jq

# Strip weapons from fashion2
fashion2_noweapons=$(gw2fashionista wardrobe filter "$fashion2" --exclude weapons)
echo $fashion2_noweapons | gw2fashionista read | jq

# Combines fashion1 weapons with fashion2
gw2fashionista wardrobe  merge "$fashion1" "$fashion2_noweapons" | gw2fashionista read | jq

# Showcase the different filtering options and outputs how it affects the backpack:

# fashion2 backpack
gw2fashionista wardrobe  merge "$fashion1" "$fashion2_noweapons" | gw2fashionista read | jq .backpack
# fashion1 backpack
gw2fashionista wardrobe  merge "$fashion1" "$fashion2_noweapons" --exclude backpack | gw2fashionista read | jq .backpack
# fashion1 backpack skin and fashion2 dyes
gw2fashionista wardrobe  merge "$fashion1" "$fashion2_noweapons" --no-skins | gw2fashionista read | jq .backpack
# fashion2 backpack skin and fashion1 dyes
gw2fashionista wardrobe  merge "$fashion1" "$fashion2_noweapons" --no-dyes | gw2fashionista read | jq .backpack
```
