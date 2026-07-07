/// Represents a wardrobe template test case
pub struct WardrobeTemplate {
    pub name: &'static str,
    pub chat_link: &'static str,
}

impl WardrobeTemplate {
    const fn new(name: &'static str, chat_link: &'static str) -> Self {
        WardrobeTemplate { name, chat_link }
    }

    pub fn snapshot_name(&self, prefix: &str) -> String {
        if prefix.is_empty() {
            self.name.to_string()
        } else {
            format!("{}_{}", prefix, self.name)
        }
    }
}

pub const ALL_TEMPLATES: &[WardrobeTemplate] = &[
    EMPTY_TEMPLATE,
    PEEKABOO_TEMPLATE,
    ZIZI_TEMPLATE,
    ZIZI_ARMOR_TEMPLATE,
];

/// Wardrobe template with nothing set
pub const EMPTY_TEMPLATE: WardrobeTemplate = WardrobeTemplate::new(
    "empty",
    "DwAAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAAAAAAAAAAAAAD/fw==",
);

/// Peekaboo's wardrobe template
pub const PEEKABOO_TEMPLATE: WardrobeTemplate = WardrobeTemplate::new(
    "peekaboo",
    "D7UfzTMeBQYA4gQGAJ4AHgUGAB4FAQCsAAYABgAeBQEANSgBAAYAHgUBAMkDHgUGAAEAAQDVAAYAHgUeBQEAoRYeBQYAAQABADIAAQABAAEAAQBoEqAPFCovKj8SAAD/fg==",
);

/// Zizï Skyhoof's wardrobe template
pub const ZIZI_TEMPLATE: WardrobeTemplate = WardrobeTemplate::new(
    "zizi",
    "D1sDPQkBAAEAAQABAAwAGAURBhEGAQAjABgFEQYBAAEA/AABABEGGAUYBdIDGAURBgEAAQALAAEAEQYRBgEAohYYBREGAQABAHwAAQABAAEAAQDjE6APPBI8Ej0SAAD+fg==",
);

/// Zizï Skyhoof's armor (+ backpack) template
pub const ZIZI_ARMOR_TEMPLATE: WardrobeTemplate = WardrobeTemplate::new(
    "zizi_armor",
    "DwAAPQkBAAEAAQABAAwAGAURBhEGAQAjABgFEQYBAAEA/AABABEGGAUYBdIDGAURBgEAAQALAAEAEQYRBgEAohYYBREGAQABAAAAAQABAAEAAQAAAAAAAAAAAAAAAAD/fw==",
);

pub fn all_templates_as_csv() -> Vec<String> {
    ALL_TEMPLATES
        .iter()
        .map(|t| format!("{},{}", t.name, t.chat_link))
        .collect()
}

pub fn all_templates_as_list() -> Vec<String> {
    ALL_TEMPLATES
        .iter()
        .map(|t| t.chat_link.to_string())
        .collect()
}
