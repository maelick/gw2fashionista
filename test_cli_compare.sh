#!/usr/bin/env bash

echo Creating output dir and cleanup
mkdir -p output
rm -f output/*

echo "Run Python CLI"
bash test_cli_python.sh
bash test_cli_rust.sh

echo "Diff peekaboo"
diff -u output/peekaboo_{py,rs}

echo "Diff zizi armor"
diff -u output/zizi_armor_{py,rs}

echo "Diff both"
diff -u output/both_{py,rs}

echo "Diff peekaboo filter out weapons"
diff -u output/peekaboo_no_weapons_{py,rs}

echo "Diff peekaboo with zizi armor"
diff -u output/peekaboo_zizi_armor_{py,rs}

echo "Diff peekaboo with zizi armor no backpack"
diff -u output/peekaboo_zizi_armor_no_backpack_{py,rs}

echo "Diff peekabo with zizi armor dyes only"
diff -u output/peekaboo_zizi_armor_dyes_only_{py,rs}

echo "Diff peekaboo with zizi armor skins only"
diff -u output/peekaboo_zizi_armor_skins_only_{py,rs}

echo "Diff export peekaboo "
diff -u output/export_peekaboo_{py,rs}

echo "Diff export all"
diff -u output/export_all_{py,rs}