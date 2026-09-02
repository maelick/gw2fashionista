use gw2fashionista_core::domain::templates::wardrobe::{WardrobeSlot, WardrobeTemplate};
use gw2fashionista_fixtures::wardrobe::{PEEKABOO_TEMPLATE, ZIZI_ARMOR_TEMPLATE};

#[test]
#[test_log::test]
fn test_merge_peekaboo_with_zizi_armor() {
    let base_template = &PEEKABOO_TEMPLATE
        .chat_link
        .parse::<WardrobeTemplate>()
        .unwrap();

    let armor_template = &ZIZI_ARMOR_TEMPLATE
        .chat_link
        .parse::<WardrobeTemplate>()
        .unwrap();

    let merged = base_template.merge(&armor_template, false, false);

    for (slot, appearance) in &merged {
        match slot {
            WardrobeSlot::Backpack
            | WardrobeSlot::Chest
            | WardrobeSlot::Shoes
            | WardrobeSlot::Gloves
            | WardrobeSlot::Head
            | WardrobeSlot::Legs
            | WardrobeSlot::Shoulders => {
                assert_eq!(appearance, armor_template.get_slot(&slot));
            }
            _ => {
                assert_eq!(appearance, base_template.get_slot(&slot));
            }
        }
    }
}

#[test]
#[test_log::test]
fn test_merge_peekaboo_with_zizi_armor_skins_only() {
    let base_template = &PEEKABOO_TEMPLATE
        .chat_link
        .parse::<WardrobeTemplate>()
        .unwrap();

    let armor_template = &ZIZI_ARMOR_TEMPLATE
        .chat_link
        .parse::<WardrobeTemplate>()
        .unwrap();

    let merged = base_template.merge(&armor_template, false, true);

    for (slot, appearance) in &merged {
        match slot {
            WardrobeSlot::Backpack
            | WardrobeSlot::Chest
            | WardrobeSlot::Shoes
            | WardrobeSlot::Gloves
            | WardrobeSlot::Head
            | WardrobeSlot::Legs
            | WardrobeSlot::Shoulders => {
                assert_eq!(appearance.skin(), armor_template.get_slot(&slot).skin());
                assert_eq!(
                    appearance.is_visible(),
                    armor_template.get_slot(&slot).is_visible()
                );
                assert_eq!(appearance.dyes(), base_template.get_slot(&slot).dyes());
            }
            _ => {
                assert_eq!(appearance, base_template.get_slot(&slot));
            }
        }
    }
}

#[test]
#[test_log::test]
fn test_merge_peekaboo_with_zizi_armor_dyes_only() {
    let base_template = &PEEKABOO_TEMPLATE
        .chat_link
        .parse::<WardrobeTemplate>()
        .unwrap();

    let armor_template = &ZIZI_ARMOR_TEMPLATE
        .chat_link
        .parse::<WardrobeTemplate>()
        .unwrap();

    let merged = base_template.merge(&armor_template, true, false);

    for (slot, appearance) in &merged {
        match slot {
            WardrobeSlot::Backpack
            | WardrobeSlot::Chest
            | WardrobeSlot::Shoes
            | WardrobeSlot::Gloves
            | WardrobeSlot::Head
            | WardrobeSlot::Legs
            | WardrobeSlot::Shoulders => {
                assert_eq!(appearance.skin(), base_template.get_slot(&slot).skin());
                assert_eq!(
                    appearance.is_visible(),
                    base_template.get_slot(&slot).is_visible()
                );
                assert_eq!(appearance.dyes(), armor_template.get_slot(&slot).dyes());
            }
            _ => {
                assert_eq!(appearance, base_template.get_slot(&slot));
            }
        }
    }
}
