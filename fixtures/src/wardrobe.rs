pub type WardrobeTemplate = super::FashionTemplate;

pub const ALL_TEMPLATES: &[WardrobeTemplate] = &[
    EMPTY_TEMPLATE,
    PEEKABOO_TEMPLATE,
    ZIZI_TEMPLATE,
    ZIZI_ARMOR_TEMPLATE,
];

/// Wardrobe template with nothing set
pub const EMPTY_TEMPLATE: WardrobeTemplate = WardrobeTemplate::new(
    "wardrobe_empty",
    "DwAAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAAAAAAAAAAAAAD/fw==",
);

/// Peekaboo's wardrobe template
pub const PEEKABOO_TEMPLATE: WardrobeTemplate = WardrobeTemplate::new(
    "wardrobe_peekaboo",
    "D7UfzTMeBQYA4gQGAJ4AHgUGAB4FAQCsAAYABgAeBQEANSgBAAYAHgUBAMkDHgUGAAEAAQDVAAYAHgUeBQEAoRYeBQYAAQABADIAAQABAAEAAQBoEqAPFCovKj8SAAD/fg==",
);

/// Zizï Skyhoof's wardrobe template
pub const ZIZI_TEMPLATE: WardrobeTemplate = WardrobeTemplate::new(
    "wardrobe_zizi",
    "D1sDPQkBAAEAAQABAAwAGAURBhEGAQAjABgFEQYBAAEA/AABABEGGAUYBdIDGAURBgEAAQALAAEAEQYRBgEAohYYBREGAQABAHwAAQABAAEAAQDjE6APPBI8Ej0SAAD+fg==",
);

/// Zizï Skyhoof's armor (+ backpack) template
pub const ZIZI_ARMOR_TEMPLATE: WardrobeTemplate = WardrobeTemplate::new(
    "wardrobe_zizi_armor",
    "DwAAPQkBAAEAAQABAAwAGAURBhEGAQAjABgFEQYBAAEA/AABABEGGAUYBdIDGAURBgEAAQALAAEAEQYRBgEAohYYBREGAQABAAAAAQABAAEAAQAAAAAAAAAAAAAAAAD/fw==",
);
