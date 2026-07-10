use clap::{Args, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Format {
    /// Based on the output filename extension (default to CSV if missing filename or unknown extension)
    Auto,
    /// CSV
    Csv,
    /// JSON
    Json,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct SkinsOrDyes {
    /// Do not merge skins (i.e. original skins will be preserved)
    #[arg(long, default_value_t = false, display_order = 20)]
    pub no_skins: bool,

    /// Do not merge dyes (i.e. original dyes will be preserved)
    #[arg(long, default_value_t = false, display_order = 20)]
    pub no_dyes: bool,
}
