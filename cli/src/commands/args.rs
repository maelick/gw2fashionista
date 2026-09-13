use std::io::{self, IsTerminal};

use clap::{Args, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum DataFormat {
    /// Auto detection
    Auto,
    /// CSV
    Csv,
    /// JSON
    Json,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum InputMode {
    /// Auto detection based on whether stdin is a terminal
    Auto,
    /// Always read from stdin
    Always,
    /// Never read from stdin
    Never,
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

impl From<&InputMode> for bool {
    fn from(value: &InputMode) -> Self {
        match value {
            InputMode::Auto => !io::stdin().is_terminal(),
            InputMode::Always => true,
            InputMode::Never => false,
        }
    }
}
