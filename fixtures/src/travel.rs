pub type TravelTemplate = super::FashionTemplate;

pub const ALL_TEMPLATES: &[TravelTemplate] = &[
    EMPTY_TEMPLATE,
    DEFAULT_MOUNT_TEMPLATE,
    NO_DYES_TEMPLATE,
    PEEKABOO_TEMPLATE,
    ZIZI_TEMPLATE,
];

/// Travel template with nothing set
pub const EMPTY_TEMPLATE: TravelTemplate = TravelTemplate::new(
    "travel_empty",
    "EAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAP8P",
);

/// Travel template with default skins with shadow abyss dye
pub const DEFAULT_MOUNT_TEMPLATE: TravelTemplate = TravelTemplate::new(
    "travel_default",
    "EAIASgUBAAEAAQACAAEAAQABAAEABgABAAEAAQBKBQQASgUBAAEAAQADAEoFAQABAAEAAgBKBQEAAQABAAEASgUBAAEAAQBnAAEAAQABAEoFoQBKBQEAAQABALoASgUBAAEAAQCaAUoFAQABAAEAjQFKBQEAAQABAP8P",
);

/// Travel template with some skins set (except turtle) with no dye set
pub const NO_DYES_TEMPLATE: TravelTemplate = TravelTemplate::new(
    "travel_no_dyes",
    "EJkAygHbAV0GOwYFAD0GgQCeAoECYQC7AXsAhgS6AvYATQAwBkwCbgKnAVcAGQVHBWQGhwHcAS4GPQY7BkkBUAEEACgGoQIlARkAaQYuBl0GwgKtBlMAYwIOBowBUQDAAUAACQDDAVUATAF5AZwC+AGZAlMCbQCIAv8P",
);

/// Peekabo's travel template
pub const PEEKABOO_TEMPLATE: TravelTemplate = TravelTemplate::new(
    "travel_peekaboo",
    "EJAABgDiBOIEHgUFAB4F4gTiBOIEbgIeBeIEHgXiBEoBBgAeBUgF4gTzAeIEHgVIBR4FJAPiBOIEHgVIBZMBBgAeBeIESAXnAkgFHgXiBOIEyAIGAB4F4gRIBXAC4gQGAAYAXwHDAR4F4gQeBeIE+AEeBeIESAUeBf8P",
);

/// Zizï Skyhoof's travel template
pub const ZIZI_TEMPLATE: TravelTemplate = TravelTemplate::new(
    "travel_zizi",
    "EJkAGAUYBREGEQYFAH4AgwAYBYECbgK7ARgFGAUYBUoBEQYYBRgFGAXzARgFEQYYBRgFJAMRBhgFfgB+AJMBEQYYBagCDQbnAhgFEQYYBdkByAIRBhgFqAJ+AHACGAURBhEGGAWdAREGGAUYBREG+AERBhgFGAURBv8P",
);
