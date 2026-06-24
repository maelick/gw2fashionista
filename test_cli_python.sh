#!/usr/bin/env bash

peekaboo='[&D7UfzTMeBQYA4gQGAJ4AHgUGAB4FAQCsAAYABgAeBQEANSgBAAYAHgUBAMkDHgUGAAEAAQDVAAYAHgUeBQEAoRYeBQYAAQABADIAAQABAAEAAQBoEqAPFCovKj8SAAD/fg==]'
zizi_armor='[&DwAAPQkBAAEAAQABAAwAGAURBhEGAQAjABgFEQYBAAEA/AABABEGGAUYBdIDGAURBgEAAQALAAEAEQYRBgEAohYYBREGAQABAAAAAQABAAEAAQAAAAAAAAAAAAAAAAD/fw==]'

CLI=./gw2fashionista-py
DYE_FILTER='walk(if type == "object" and has("dye1") then (. + {dyes: [.dye1, .dye2, .dye3, .dye4] | map(select(. != null))}) | del(.dye1, .dye2, .dye3, .dye4) else . end)'

$CLI read "$peekaboo" | jq '.[].skins' | jq -S "$DYE_FILTER" > output/peekaboo_py
$CLI read "$zizi_armor" | jq '.[].skins' | jq -S "$DYE_FILTER" > output/zizi_armor_py
$CLI read "$peekaboo" "$zizi_armor" | jq '.[].skins' | jq -S "$DYE_FILTER" > output/both_py

$CLI filter "$peekaboo" --no-weapons | $CLI read | jq '.[].skins' | jq -S "$DYE_FILTER" > output/peekaboo_no_weapons_py
$CLI merge "$peekaboo" "$zizi_armor" | $CLI read | jq '.[].skins' | jq -S "$DYE_FILTER" > output/peekaboo_zizi_armor_py
$CLI merge "$peekaboo" "$zizi_armor" --no-backpack | $CLI read | jq '.[].skins' | jq -S "$DYE_FILTER" > output/peekaboo_zizi_armor_no_backpack_py
$CLI merge "$peekaboo" "$zizi_armor" --dyes-only | $CLI read | jq '.[].skins' | jq -S "$DYE_FILTER" > output/peekaboo_zizi_armor_dyes_only_py
$CLI merge "$peekaboo" "$zizi_armor" --skin-only | $CLI read | jq '.[].skins' | jq -S "$DYE_FILTER" > output/peekaboo_zizi_armor_skins_only_py

$CLI export "Pikku Peekaboo" > output/export_peekaboo_py
time $CLI -v export > output/export_all_py
