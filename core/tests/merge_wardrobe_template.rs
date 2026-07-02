#[cfg(test)]
mod tests {
    use gw2fashionista_core::domain::{chatlink::ChatLink, wardrobe_template::slot::SlotType};
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

        for (slot_type, slot) in &merged {
            match slot_type {
                SlotType::Backpack
                | SlotType::Chest
                | SlotType::Shoes
                | SlotType::Gloves
                | SlotType::Head
                | SlotType::Legs
                | SlotType::Shoulders => {
                    assert_eq!(slot, armor_template.get_slot(&slot_type));
                }
                _ => {
                    assert_eq!(slot, base_template.get_slot(&slot_type));
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

        for (slot_type, slot) in &merged {
            match slot_type {
                SlotType::Backpack
                | SlotType::Chest
                | SlotType::Shoes
                | SlotType::Gloves
                | SlotType::Head
                | SlotType::Legs
                | SlotType::Shoulders => {
                    assert_eq!(slot.skin(), armor_template.get_slot(&slot_type).skin());
                    assert_eq!(
                        slot.is_visible(),
                        armor_template.get_slot(&slot_type).is_visible()
                    );
                    assert_eq!(slot.dyes(), base_template.get_slot(&slot_type).dyes());
                }
                _ => {
                    assert_eq!(slot, base_template.get_slot(&slot_type));
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

        for (slot_type, slot) in &merged {
            match slot_type {
                SlotType::Backpack
                | SlotType::Chest
                | SlotType::Shoes
                | SlotType::Gloves
                | SlotType::Head
                | SlotType::Legs
                | SlotType::Shoulders => {
                    assert_eq!(slot.skin(), base_template.get_slot(&slot_type).skin());
                    assert_eq!(
                        slot.is_visible(),
                        base_template.get_slot(&slot_type).is_visible()
                    );
                    assert_eq!(slot.dyes(), armor_template.get_slot(&slot_type).dyes());
                }
                _ => {
                    assert_eq!(slot, base_template.get_slot(&slot_type));
                }
            }
        }
    }
}
