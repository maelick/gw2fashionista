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

impl From<&EquipmentFilters> for SlotFilter {
    fn from(value: &EquipmentFilters) -> Self {
        let mut filter = SlotFilter::all();
        if value.no_weapons {
            filter.filter_out(EquipmentCategory::Weapon);
        }
        if value.no_armor {
            filter.filter_out(EquipmentCategory::Armor);
        }
        if value.no_backpack {
            filter.remove(&SlotType::Backpack);
        }
        if value.no_outfit {
            filter.remove(&SlotType::Outfit);
        }

        let underwater = (&value.underwater).into();
        filter.intersection(&underwater).copied().collect()
    }
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

impl From<&UnderwaterFilters> for SlotFilter {
    fn from(value: &UnderwaterFilters) -> Self {
        let mut filter = SlotFilter::all();
        if value.no_underwater {
            filter.filter_out(EquipmentCategory::Underwater);
        }
        if value.only_underwater {
            filter.keep_only(EquipmentCategory::Underwater);
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
