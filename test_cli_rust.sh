#!/usr/bin/env bash

peekaboo='[&D7UfzTMeBQYA4gQGAJ4AHgUGAB4FAQCsAAYABgAeBQEANSgBAAYAHgUBAMkDHgUGAAEAAQDVAAYAHgUeBQEAoRYeBQYAAQABADIAAQABAAEAAQBoEqAPFCovKj8SAAD/fg==]'
zizi_armor='[&DwAAPQkBAAEAAQABAAwAGAURBhEGAQAjABgFEQYBAAEA/AABABEGGAUYBdIDGAURBgEAAQALAAEAEQYRBgEAohYYBREGAQABAAAAAQABAAEAAQAAAAAAAAAAAAAAAAD/fw==]'

CLI=./gw2fashionista-rs

$CLI read "$peekaboo" | jq -S > output/peekaboo_rs
$CLI read "$zizi_armor" | jq -S > output/zizi_armor_rs
$CLI read "$peekaboo" "$zizi_armor" | jq -S > output/both_rs

$CLI wardrobe filter "$peekaboo" --exclude weapons | $CLI read | jq -S > output/peekaboo_no_weapons_rs
$CLI wardrobe merge "$peekaboo" "$zizi_armor" | $CLI read | jq -S > output/peekaboo_zizi_armor_rs
$CLI wardrobe merge "$peekaboo" "$zizi_armor" --exclude backpack | $CLI read | jq -S > output/peekaboo_zizi_armor_no_backpack_rs
$CLI wardrobe merge "$peekaboo" "$zizi_armor" --no-skins | $CLI read | jq -S > output/peekaboo_zizi_armor_dyes_only_rs
$CLI wardrobe merge "$peekaboo" "$zizi_armor" --no-dyes | $CLI read | jq -S > output/peekaboo_zizi_armor_skins_only_rs

dotenv-rust $CLI wardrobe export "Pikku Peekaboo" > output/export_peekaboo_rs
time dotenv-rust $CLI wardrobe export > output/export_all_rs
