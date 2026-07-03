use std::str::FromStr;

use clap::{Args, ValueEnum, builder::TypedValueParser};
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

use gw2fashionista_core::domain::templates::{
    SlotFilter, SlotFilterExt,
    wardrobe::slot::{EquipmentCategory, SlotType},
};

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

static FILTER_VARIANTS: Lazy<Vec<FilterOption>> = Lazy::new(|| {
    let categories = EquipmentCategory::iter().map(FilterOption::Category);
    let slots = SlotType::iter().map(FilterOption::Slot);
    categories.chain(slots).collect()
});

impl ValueEnum for FilterOption {
    fn value_variants<'a>() -> &'a [Self] {
        &FILTER_VARIANTS
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            FilterOption::Category(cat) => clap::builder::PossibleValue::new(cat.to_string()),
            FilterOption::Slot(slot) => clap::builder::PossibleValue::new(slot.to_string()),
        })
    }
}

impl TypedValueParser for FilterOption {
    type Value = Self;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        value
            .to_string_lossy()
            .parse()
            .map_err(|_| clap::Error::raw(clap::error::ErrorKind::InvalidValue, "Invalid filter"))
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = clap::builder::PossibleValue> + '_>> {
        let categories = EquipmentCategory::iter()
            .map(|c| clap::builder::PossibleValue::new(format!("{:?}", c).to_lowercase()));
        let slots = SlotType::iter()
            .map(|s| clap::builder::PossibleValue::new(format!("{:?}", s).to_lowercase()));

        Some(Box::new(categories.chain(slots)))
    }
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

impl From<&EquipmentFilters> for SlotFilter<SlotType> {
    fn from(value: &EquipmentFilters) -> Self {
        let mut filter = SlotFilter::<SlotType>::all();
        for f in &value.only {
            match f {
                FilterOption::Category(category) => filter.retain_all(category.slots()),
                FilterOption::Slot(slot) => {
                    filter.retain(|s| s == slot);
                }
            };
        }

        for f in &value.exclude {
            match f {
                FilterOption::Category(category) => filter.remove_all(category.slots()),
                FilterOption::Slot(slot) => {
                    filter.remove(slot);
                }
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
