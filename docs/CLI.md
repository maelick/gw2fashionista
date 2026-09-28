# GW2 Fashionsta Command Line Interface

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

The `help` command can be used to get general help, and `help <subcommand>` to get help on a specific subcommand

## Features

### Storing and managing fashion templates

The CLI can store "fashion templates", which contain a wardrobe and travel template annotated with a name,
and optionally a character name, a description, and tags.

The templates are stored in an SQLite database that is stored in the user's application data folder by default.
A path can also be provided using the --db argument (e.g., `gw2fashionista --db <path/to/db.sqlite> fashion list`).

A fashion template has:
* A unique ID (automatically generated)
* A name and optionally a character name. The combination of both the name and the character name must be unique.
* An optional description
* A wardrobe template
* A travel template
* Optional tags

#### Managing fashion templates

The following commands can be used to manage fashion templates:
* `fashion create`: create a new fashion template
* `fashion get`: retrieve an existing fashion template using its id or name/character name
* `fashion set`: update an existing fashion template, overriding all fields
* `fashion patch`: update an existing fashion template, overriding only the provided fields
* `fashion delete`: delete an existing fashion template
* `fashion list`: list all fashion templates
* `fashion untag`: remove all tags from a fashion template

Most of these commands work with structured data that can be formatted as JSON or CSV.
By default, the data format is auto-detected based on whether stdin/stdout are a TTY,
but it can be manually specified using command-line arguments.

The commands that work with an existing template can identify it using either its unique ID or the couple name/character name.

#### Managing tags

* `fashion tag list`: list all available tags and display how many templates use them
* `fashion tag renane`: rename a tag (the new tag name must not be used by another tag yet)
* `fashion tag replace`: replace a tag with another one (does not delete the original tag)
* `fashion tag delete`: delete a tag
* `fashion tag clean`: remove unused tags from the database

#### Retrieving or updating a wardrobe or travel template

The following commands can be used to retrieve or update a wardrobe and travel template:
* `fashion wardrobe get/set`
* `fashion travel get/set`

These commands always read/write the chat link on stdin/stdout as a raw string.
Their main purpose is to be used and piped with other commands
 such as `read` and `wardrobe/travel filter/merge` (see below).

### Exporting equipment tabs as wardrobe templates

Equipment tabs can be retrieved from GW2 using the API and converted to wardrobe templates.
This requires an API key with the `account`, `builds`, and `characters` permissions
to be provided as an environment variable.

For example, to export all characters to wardrobe.csv:
```bash
export GW2_API_KEY='<your-api-key-here>'
./gw2fashionista wardrobe export -o wardrobe.csv
```

### Reading chat links

The CLI can be used to read one or several
[GW2 chat link](https://wiki.guildwars2.com/wiki/Chat_link_format)
and output its content as JSON.

The chat links can be passed as command arguments or on the standard input (one per line or as a CSV file).
By default, it also resolves skin/outfit/dye names using the GW2 API.

For example, to read all exported equipment tabs (see previous section):
```bash
./gw2fashionista read < wardrobe.csv
```

Note: this command currently only supports wardrobe and travel templates.

### Transforming wardrobe and travel templates

Wardrobe and travel templates can be filtered and merged together.

Filtering takes one chat link as input and outputs a new chat link
* that only include specific skins or skin categories
* or from which specific skins or skin categories have been removed

For example, to remove the weapons of a wardrobe template:
```bash
./gw2fashionista wardrobe filter "$fashion2" --exclude weapons
```

Merging takes two chat links as input, and outputs a new chat link for which the non-empty skins and/or dye slots
of the second template have been applied to the first one.
The command takes similar options for filtering skins being merged.
The merging can also be limited to dyes or skins.

For example, to apply the armor dyes of the second chat link to the first:
```bash
fashion1='[&<base64-encoded-template>]'
fashion2='[&<base64-encoded-template]]'
./gw2fashionista wardrobe merge "$fashion1" "$fashion2" --no-skins --only armor
```

## Examples

The following examples are written for Linux and/or bash terminals.

```bash
fashion1='[&<base64-encoded-template>]'
fashion2='[&<base64-encoded-template]]'
./gw2fashionista wardrobe merge "$fashion1" "$fashion2_noweapons"

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
