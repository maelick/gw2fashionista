use gw2lib::model::{
    items::{Item, skins::Skin},
    misc::colors::Color,
};

use crate::gw2::endpoints::{glider::Glider, mount::MountSkin, outfit::Outfit, skiff::Skiff};

pub trait Named {
    fn name(&self) -> &str;
}

impl Named for Item {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Named for Skin {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Named for Outfit {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Named for Color {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Named for MountSkin {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Named for Glider {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Named for Skiff {
    fn name(&self) -> &str {
        &self.name
    }
}
