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

The templates are stored in an SQLite database that is stored in the user's application data folder by default
(e.g. `~/.local/share/gw2fashionista/gw2fashionista.sqlite` on Linux).
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
* `fashion import`: import a list of templates read from stdin with different modes to handle already existing templates with the same name/character:
  * overwrite: overwrite the existing template(s) with the imported one (i.e. will remove description, tags and travel template)
  * patch: patch the existing template(s) with the imported one (i.e. will overwrite the wardrobe template but will keep existing description, tags and travel template)
  * skip: import non-existing fashion, print a warning for existing ones and exit with a status error
  * fail (default): abort the command if there is at least one template which name and character name matches
  * rename: rename any pre-existing template by adding a number at the end and retries until a non-used named is found

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
./gw2fashionista read --column wardrobe_template < wardrobe.csv
```

Note: this command does not currently support chat links for
build templates (not yet implemented) and PvP games (format not documented).

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

### Transforming wardrobe and travel template chat links

```bash
fashion1='[&<base64-encoded-template>]'
fashion2='[&<base64-encoded-template]]'

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

### Managing fashion templates

#### Creating, reading, updating and deleting templates

```bash
# Listing existing fashion templates in the database (empty if new database)
./gw2fashionista fashion list

# Adding a new empty fashion with a description and two tags (empty and todo)
./gw2fashionista fashion create -n "First Template" -d "The template's description" -t empty -t todo

# Retrieving the fashion template in the database
./gw2fashionista fashion get -n "First Template"

# Updating some of the template's fields without erasing the other ones
./gw2fashionista fashion patch -n "First Template" --wardrobe "[&<base64-encoded-template>]"
./gw2fashionista fashion patch -n "First Template" --travel "[&<base64-encoded-template>]"

# Update the template and erasing the fields not provided
./gw2fashionista fashion set -n "First Template" --wardrobe "[&<base64-encoded-template>]"

# Deleting the template
./gw2fashionista fashion delete -n "First Template"
```

#### Associating templates with characters

A fashion template can be associated with a specific character.
The character name becomes then part of the fashion's identifier.

```bash
# Adding a new fashion for specific characters with a wardrobe and travel templates
./gw2fashionista fashion create -n "Second Template" -c "My Character Name" --wardrobe "[&<base64-encoded-template>]" --travel "[&<base64-encoded-template>]"
./gw2fashionista fashion create -n "Second Template" -c "My Second Character Name" --wardrobe "[&<base64-encoded-template>]" --travel "[&<base64-encoded-template>]"
# Retrieving the created fashion templates
./gw2fashionista fashion get -n "Second Template" -c "My Character Name"
./gw2fashionista fashion get -n "Second Template" -c "My Second Character Name"
# Retrieving the template without a character will fail
./gw2fashionista fashion get -n "Second Template"
# We can also create a template with the same name without a character
./gw2fashionista fashion create -n "Second Template" --wardrobe "[&<base64-encoded-template>]" --travel "[&<base64-encoded-template>]"
# Retrieving the template without a character will now work
./gw2fashionista fashion get -n "Second Template"
```

#### Tag management

```bash
# Creating several templates with different tags
./gw2fashionista fashion create -n template1 -t tag1 -t tag2
./gw2fashionista fashion create -n template2 -t tag2 -t tag3 -t tag4
./gw2fashionista fashion create -n template3 -t tag3 -t tag4 -t tag5
./gw2fashionista fashion create -n template4 -t hello1 -t hello2 -t hello3

# Adding a new tag to a template
./gw2fashionista fashion patch -n template1 -t tag3

# Replacing all tags for a template
./gw2fashionista fashion set -n template1 -t tag2 -t tag4

# Removing all tags from a template
./gw2fashionista fashion untag -n template1

# Replacing all usage of tag4, tag5 and hello1 with tag6
./gw2fashionista fashion tag replace tag4 tag5 hello1 tag6

# Renaming tag6 as tag7
./gw2fashionista fashion tag rename tag6 tag7

# See the list of tags and the number of templates using them
./gw2fashionista fashion tag list

# Cleaning up unused tags
./gw2fashionista fashion tag clean

# Deleting tags
./gw2fashionista fashion tag delete tag7 hello3

# Listing templates with one or several tag(s)
./gw2fashionista fashion list tag1
./gw2fashionista fashion list tag3
./gw2fashionista fashion list tag2 tag3
```

### Piping commands

Most commands can read data from the standard input rather than from the command parameters,
making it possible to pipe several commands together for running more complex operations.

```bash
# Adding a tag to all templates with a specific tag
./gw2fashionista fashion list tag3 | ./gw2fashionista fashion patch -t peekaboo

# Replacing all tags with a single tag for all templates with a specific tag.
# Note that when using `set`, we need to filter the resulting JSON to keep only the fashion id, name and character,
# otherwise we would update each template with all its fields.
./gw2fashionista fashion list tag3 | jq '[.[] | {id,name,character}]' | ./gw2fashionista fashion set -t peekaboo2

# Deleting all templates with a specific tag
./gw2fashionista fashion list tag3 | ./gw2fashionista fashion delete
```

The subcommands `fashion wardrobe|travel get|set` are specifically designed to read/write
a chat link from/to the standard input/output so they can be modified with other commands
such as `wardrobe|travel filter`:

```bash
./gw2fashionista fashion create -n "A non empty fashion" --wardrobe "[&<base64-encoded-template>]" --travel "[&<base64-encoded-template>]"
./gw2fashionista fashion create -n "An empty fashion"

# Apply the travel template of the first fashion to the second
./gw2fashionista fashion travel get -n "A non empty fashion" | \
  ./gw2fashionista fashion travel set -n "An empty fashion"

# Filter and update the first template's wardrobe chat link
./gw2fashionista fashion wardrobe get -n "A non empty fashion" | \
  ./gw2fashionista fashion filter --exclude outfit,underwater,weapons | \
  ./gw2fashionista fashion wardrobe get -n "A non empty fashion"
```

## Importing exported equipment tabs as fashion templates

The `wardrobe export` command outputs data in a format that can be understood by `fashion import`.

```bash
# Let's pretend we have one character's fashion already saved:
./gw2fashionista fashion create -n "Equipment tab 1" -c "My Cool Character" -d "Coolest fashion ever" --wardrobe "[&<base64-encoded-template>]" -t cool

# Assuming we don't already have any template for the account's characters and/or equipment tabs,
# we can import all characters current equipment tabs and skip the existing one like this:
./gw2fashionista wardrobe export | ./gw2fashionista fashion import --mode skip

# Trying to import it with the default mode (fail) prevents from importing anything
./gw2fashionista wardrobe export | ./gw2fashionista fashion import

# Let's modify an imported fashion:
./gw2fashionista fashion patch -n "Equipment tab 1" -c "My Other Cool Character" -d "This one has also a travel template" --travel "[&<base64-encoded-template>]" -t cool

# We can also improt and patch existing templates to avoid losing the descriptions, tags and travel templates
./gw2fashionista wardrobe export | ./gw2fashionista fashion import --mode patch

# We can rename the duplicate templates to import to avoid overwriting anything
./gw2fashionista wardrobe export | ./gw2fashionista fashion import --mode rename

# We can do this as many times as we want
./gw2fashionista wardrobe export | ./gw2fashionista fashion import --mode rename
./gw2fashionista wardrobe export | ./gw2fashionista fashion import --mode rename

# Finally we can also decide to overwrite the existing ones and lose any of our changes
./gw2fashionista wardrobe export | ./gw2fashionista fashion import --mode overwrite
```