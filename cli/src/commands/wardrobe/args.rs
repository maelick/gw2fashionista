use std::{str::FromStr, sync::LazyLock};

use clap::{Args, ValueEnum, builder::TypedValueParser};
use linearize::LinearizeExt;

use gw2fashionista_core::domain::templates::{
    SlotFilter, SlotFilterExt,
    wardrobe::{EquipmentCategory, WardrobeSlot},
};

#[derive(Args, Debug)]
#[group(multiple = true)]
pub struct WardrobeFilters {
    /// Only keep the provided comma-separated skins (or skin categories)
    #[arg(long, value_delimiter = ',')]
    only: Vec<WardrobeFilterOption>,

    /// Exclude the provided comma-separated skins (or skin categories)
    #[arg(long, value_delimiter = ',')]
    exclude: Vec<WardrobeFilterOption>,
}

#[derive(Debug, Clone)]
enum WardrobeFilterOption {
    Category(EquipmentCategory),
    Slot(WardrobeSlot),
}

static FILTER_VARIANTS: LazyLock<Vec<WardrobeFilterOption>> = LazyLock::new(|| {
    let categories = EquipmentCategory::variants().map(WardrobeFilterOption::Category);
    let slots = WardrobeSlot::variants().map(WardrobeFilterOption::Slot);
    categories.chain(slots).collect()
});

impl ValueEnum for WardrobeFilterOption {
    fn value_variants<'a>() -> &'a [Self] {
        &FILTER_VARIANTS
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            WardrobeFilterOption::Category(cat) => {
                clap::builder::PossibleValue::new(cat.to_string())
            }
            WardrobeFilterOption::Slot(slot) => clap::builder::PossibleValue::new(slot.to_string()),
        })
    }
}

impl TypedValueParser for WardrobeFilterOption {
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
        let categories = EquipmentCategory::variants()
            .map(|c| clap::builder::PossibleValue::new(format!("{:?}", c).to_lowercase()));
        let slots = WardrobeSlot::variants()
            .map(|s| clap::builder::PossibleValue::new(format!("{:?}", s).to_lowercase()));

        Some(Box::new(categories.chain(slots)))
    }
}

impl FromStr for WardrobeFilterOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try parsing as category first
        if let Ok(category) = EquipmentCategory::from_str(s) {
            return Ok(WardrobeFilterOption::Category(category));
        }
        // Then try as slot
        if let Ok(slot) = WardrobeSlot::from_str(s) {
            return Ok(WardrobeFilterOption::Slot(slot));
        }
        Err(format!("Unknown filter: {}", s))
    }
}

impl From<&WardrobeFilters> for SlotFilter<WardrobeSlot> {
    fn from(value: &WardrobeFilters) -> Self {
        let mut filter = SlotFilter::<WardrobeSlot>::all();
        for f in &value.only {
            match f {
                WardrobeFilterOption::Category(category) => filter.retain_all(category.slots()),
                WardrobeFilterOption::Slot(slot) => {
                    filter.retain(|s| s == slot);
                }
            };
        }

        for f in &value.exclude {
            match f {
                WardrobeFilterOption::Category(category) => filter.remove_all(category.slots()),
                WardrobeFilterOption::Slot(slot) => {
                    filter.remove(slot);
                }
            };
        }
        filter
    }
}
