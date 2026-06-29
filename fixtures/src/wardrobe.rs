/// Represents a wardrobe template test case
pub struct WardrobeTemplate {
    pub name: &'static str,
    pub chat_link: &'static str,
}

impl WardrobeTemplate {
    const fn new(name: &'static str, chat_link: &'static str) -> Self {
        WardrobeTemplate{
            name: name,
            chat_link: chat_link,
        }
    }

    pub fn snapshot_name(&self, prefix: &str) -> String {
        if prefix.is_empty() {
            self.name.to_string()
        } else {
            format!("{}_{}", prefix, self.name)
        }
    }
}

pub const ZIZI_TEMPLATE: WardrobeTemplate = WardrobeTemplate::new("zizi", "D1sDPQkBAAEAAQABAAwAGAURBhEGAQAjABgFEQYBAAEA/AABABEGGAUYBdIDGAURBgEAAQALAAEAEQYRBgEAohYYBREGAQABAHwAAQABAAEAAQDjE6APPBI8Ej0SAAD+fg==");
pub const ZIZI_ARMOR_TEMPLATE: WardrobeTemplate = WardrobeTemplate::new("zizi_armor", "DwAAPQkBAAEAAQABAAwAGAURBhEGAQAjABgFEQYBAAEA/AABABEGGAUYBdIDGAURBgEAAQALAAEAEQYRBgEAohYYBREGAQABAAAAAQABAAEAAQAAAAAAAAAAAAAAAAD/fw==");
pub const PEEKABOO_TEMPLATE: WardrobeTemplate = WardrobeTemplate::new("peekaboo", "D7UfzTMeBQYA4gQGAJ4AHgUGAB4FAQCsAAYABgAeBQEANSgBAAYAHgUBAMkDHgUGAAEAAQDVAAYAHgUeBQEAoRYeBQYAAQABADIAAQABAAEAAQBoEqAPFCovKj8SAAD/fg==");
pub const EMPTY_TEMPLATE: WardrobeTemplate = WardrobeTemplate::new("empty", "DwAAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAAAAAAAAAAAAAD/fw==");
