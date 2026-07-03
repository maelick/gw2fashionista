#[cfg(test)]
mod tests {
    use gw2fashionista_core::domain::{
        chatlink::ChatLink, templates::wardrobe::slot::WardrobeSlot,
    };
    use gw2fashionista_fixtures::wardrobe::{PEEKABOO_TEMPLATE, ZIZI_ARMOR_TEMPLATE};

    #[test]
    fn test_filter_zizi() {
        let chat_link = &ChatLink::try_from(PEEKABOO_TEMPLATE.chat_link).unwrap();
        let ChatLink::WardrobeTemplate(base_template) = chat_link else {
            panic!("Expected WardrobeTemplate, got {chat_link:?}");
        };

        let chat_link = &ChatLink::try_from(ZIZI_ARMOR_TEMPLATE.chat_link).unwrap();
        let ChatLink::WardrobeTemplate(armor_template) = chat_link else {
            panic!("Expected WardrobeTemplate, got {chat_link:?}");
        };

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
    fn test_filter_zizi_skins_only() {
        let chat_link = &ChatLink::try_from(PEEKABOO_TEMPLATE.chat_link).unwrap();
        let ChatLink::WardrobeTemplate(base_template) = chat_link else {
            panic!("Expected WardrobeTemplate, got {chat_link:?}");
        };

        let chat_link = &ChatLink::try_from(ZIZI_ARMOR_TEMPLATE.chat_link).unwrap();
        let ChatLink::WardrobeTemplate(armor_template) = chat_link else {
            panic!("Expected WardrobeTemplate, got {chat_link:?}");
        };

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
    fn test_filter_zizi_dyes_only() {
        let chat_link = &ChatLink::try_from(PEEKABOO_TEMPLATE.chat_link).unwrap();
        let ChatLink::WardrobeTemplate(base_template) = chat_link else {
            panic!("Expected WardrobeTemplate, got {chat_link:?}");
        };

        let chat_link = &ChatLink::try_from(ZIZI_ARMOR_TEMPLATE.chat_link).unwrap();
        let ChatLink::WardrobeTemplate(armor_template) = chat_link else {
            panic!("Expected WardrobeTemplate, got {chat_link:?}");
        };

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
}
