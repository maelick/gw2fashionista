use std::str::FromStr;

use clap::{Args, ValueEnum};

use gw2fashionista_core::domain::wardrobe_template::slot::{EquipmentCategory, SlotFilter, SlotFilterExt, SlotType};

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
pub struct EquipmentFilters {
    /// Only keep the provided comma-separated skins (or skin categories)
    #[arg(long, value_delimiter = ',')]
    only: Vec<FilterOption>,

    /// Exclude the provided comma-separated skins (or skin categories)
    #[arg(long, value_delimiter = ',')]
    exclude: Vec<FilterOption>,
}

#[derive(Debug, Clone)]
enum FilterOption {
    Category(EquipmentCategory),
    Slot(SlotType),
}

impl FromStr for FilterOption {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try parsing as category first
        if let Ok(category) = EquipmentCategory::from_str(s) {
            return Ok(FilterOption::Category(category));
        }
        // Then try as slot
        if let Ok(slot) = SlotType::from_str(s) {
            return Ok(FilterOption::Slot(slot));
        }
        Err(format!("Unknown filter: {}", s))
    }
}

impl From<&EquipmentFilters> for SlotFilter {
    fn from(value: &EquipmentFilters) -> Self {
        let mut filter = SlotFilter::all();
        for f in &value.only {
            match f {
                FilterOption::Category(category) => filter.retain_all(category.slots()),
                FilterOption::Slot(slot) => {
                    filter.retain(|s| s == slot);
                },
            };
        }

        for f in &value.exclude {
            match f {
                FilterOption::Category(category) => filter.remove_all(category.slots()),
                FilterOption::Slot(slot) => {
                    filter.remove(&slot);
                },
            };
        }
        filter
    }
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
