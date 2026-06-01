use clap::{Args, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Format {
    /// Based on the output filename extension (default to CSV if missing filename or unknown extension)
    Auto,
    /// CSV
    CSV,
    /// JSON
    JSON
}

#[derive(Args, Debug)]
#[group(multiple = true)]
pub struct SkinFilters {
    /// Filter out weapon skins
    #[arg(long, default_value_t = false, display_order = 10)]
    no_weapons: bool,

    /// Filter out armor skins
    #[arg(long, default_value_t = false, display_order = 10)]
    no_armor: bool,

    /// Filter out backpack skin
    #[arg(long, default_value_t = false, display_order = 10)]
    no_backpack: bool,

    /// Filter out outfit
    #[arg(long, default_value_t = false, display_order = 10)]
    no_outfit: bool,

    #[command(flatten)]
    underwater: UnderwaterFilters,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct UnderwaterFilters {
    /// Filter out underwater skins
    #[arg(long, default_value_t = false, display_order = 11)]
    no_underwater: bool,

    /// Filter out overland skins
    #[arg(long, default_value_t = false, display_order = 11)]
    only_underwater: bool,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct SkinsOrDyes {
    /// Only merge dyes (i.e. original skin will be preserved)
    #[arg(long, default_value_t = false, display_order = 20)]
    dyes_only: bool,

    /// Only merge skin (i.e. original dyes will be preserved)
    #[arg(long, default_value_t = false, display_order = 20)]
    skins_only: bool,
}
