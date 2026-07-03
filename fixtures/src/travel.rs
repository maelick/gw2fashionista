/// Represents a travel template test case
pub struct TravelTemplate {
    pub name: &'static str,
    pub chat_link: &'static str,
}

impl TravelTemplate {
    const fn new(name: &'static str, chat_link: &'static str) -> Self {
        TravelTemplate { name, chat_link }
    }

    pub fn snapshot_name(&self, prefix: &str) -> String {
        if prefix.is_empty() {
            self.name.to_string()
        } else {
            format!("{}_{}", prefix, self.name)
        }
    }
}

pub const ALL_TEMPLATES: &[TravelTemplate] = &[EMPTY_TEMPLATE, PEEKABOO_TEMPLATE, ZIZI_TEMPLATE];

pub const ZIZI_TEMPLATE: TravelTemplate = TravelTemplate::new(
    "zizi",
    "EJkAGAUYBREGEQYFAH4AgwAYBYECbgK7ARgFGAUYBUoBEQYYBRgFGAXzARgFEQYYBRgFJAMRBhgFfgB+AJMBEQYYBagCDQbnAhgFEQYYBdkByAIRBhgFqAJ+AHACGAURBhEGGAWdAREGGAUYBREG+AERBhgFGAURBv8P",
);
pub const PEEKABOO_TEMPLATE: TravelTemplate = TravelTemplate::new(
    "peekaboo",
    "EJAABgDiBOIEHgUFAB4F4gTiBOIEbgIeBeIEHgXiBEoBBgAeBUgF4gTzAeIEHgVIBR4FJAPiBOIEHgVIBZMBBgAeBeIESAXnAkgFHgXiBOIEyAIGAB4F4gRIBXAC4gQGAAYAXwHDAR4F4gQeBeIE+AEeBeIESAUeBf8P",
);
pub const EMPTY_TEMPLATE: TravelTemplate = TravelTemplate::new(
    "empty",
    "EAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAP8P",
);
