use std::{str::FromStr, sync::LazyLock};

use clap::{Args, ValueEnum, builder::TypedValueParser};
use linearize::LinearizeExt;

use gw2fashionista_core::domain::templates::{
    SlotFilter, SlotFilterExt,
    travel::{TravelCategory, TravelSlot},
};

#[derive(Args, Debug)]
#[group(multiple = true)]
pub struct TravelFilters {
    /// Only keep the provided comma-separated skins (or skin categories)
    #[arg(long, value_delimiter = ',')]
    only: Vec<TravelFilterOption>,

    /// Exclude the provided comma-separated skins (or skin categories)
    #[arg(long, value_delimiter = ',')]
    exclude: Vec<TravelFilterOption>,
}

#[derive(Debug, Clone)]
enum TravelFilterOption {
    Category(TravelCategory),
    Slot(TravelSlot),
}

static FILTER_VARIANTS: LazyLock<Vec<TravelFilterOption>> = LazyLock::new(|| {
    let categories = TravelCategory::variants().map(TravelFilterOption::Category);
    let slots = TravelSlot::variants().map(TravelFilterOption::Slot);
    categories.chain(slots).collect()
});

impl ValueEnum for TravelFilterOption {
    fn value_variants<'a>() -> &'a [Self] {
        &FILTER_VARIANTS
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            TravelFilterOption::Category(cat) => clap::builder::PossibleValue::new(cat.to_string()),
            TravelFilterOption::Slot(slot) => clap::builder::PossibleValue::new(slot.to_string()),
        })
    }
}

impl TypedValueParser for TravelFilterOption {
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
        let categories = TravelCategory::variants()
            .map(|c| clap::builder::PossibleValue::new(format!("{:?}", c).to_lowercase()));
        let slots = TravelSlot::variants()
            .map(|s| clap::builder::PossibleValue::new(format!("{:?}", s).to_lowercase()));

        Some(Box::new(categories.chain(slots)))
    }
}

impl FromStr for TravelFilterOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try parsing as category first
        if let Ok(category) = TravelCategory::from_str(s) {
            return Ok(TravelFilterOption::Category(category));
        }
        // Then try as slot
        if let Ok(slot) = TravelSlot::from_str(s) {
            return Ok(TravelFilterOption::Slot(slot));
        }
        Err(format!("Unknown filter: {}", s))
    }
}

impl From<&TravelFilters> for SlotFilter<TravelSlot> {
    fn from(value: &TravelFilters) -> Self {
        let mut filter = SlotFilter::<TravelSlot>::all();
        for f in &value.only {
            match f {
                TravelFilterOption::Category(category) => filter.retain_all(category.slots()),
                TravelFilterOption::Slot(slot) => {
                    filter.retain(|s| s == slot);
                }
            };
        }

        for f in &value.exclude {
            match f {
                TravelFilterOption::Category(category) => filter.remove_all(category.slots()),
                TravelFilterOption::Slot(slot) => {
                    filter.remove(slot);
                }
            };
        }
        filter
    }
}
