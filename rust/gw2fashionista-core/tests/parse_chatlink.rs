#[cfg(test)]
mod tests {
    use gw2fashionista_core::domain::{chatlink::ChatLink, error::ChatLinkError, wardrobe_template::{WardrobeTemplate, slot::{SlotType, EquipmentSlot}}};
use strum::IntoEnumIterator;
    use std::assert_matches;

    const EMPTY_TEMPLATE: &str = "DwAAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAAAAAAAAAAAAAD/fw==";
    const ZIZI_TEMPLATE: &str = "D1sDPQkBAAEAAQABAAwAGAURBhEGAQAjABgFEQYBAAEA/AABABEGGAUYBdIDGAURBgEAAQALAAEAEQYRBgEAohYYBREGAQABAHwAAQABAAEAAQDjE6APPBI8Ej0SAAD+fg==";
    const ZIZI_ARMOR_TEMPLATE: &str = "DwAAPQkBAAEAAQABAAwAGAURBhEGAQAjABgFEQYBAAEA/AABABEGGAUYBdIDGAURBgEAAQALAAEAEQYRBgEAohYYBREGAQABAAAAAQABAAEAAQAAAAAAAAAAAAAAAAD/fw==";
    
    #[test]
    fn test_parse_empty_link() {
        let raw = "";
        
        let result = ChatLink::try_from(raw);
        assert_matches!(result, Err(ChatLinkError::EmptyPayload));

        let raw_with_brackets = format!("[&{}]", raw);
        let result_with_brackets = ChatLink::try_from(raw_with_brackets.as_str());
        assert_matches!(result_with_brackets, Err(ChatLinkError::EmptyPayload));
    }

    #[test]
    fn test_parse_not_chat_link() {
        let raw = "This is not a chat link";
        
        let result = ChatLink::try_from(raw);
        assert_matches!(result, Err(ChatLinkError::InvalidString));

        let raw_with_brackets = format!("[&{}]", raw);
        let result_with_brackets = ChatLink::try_from(raw_with_brackets.as_str());
        assert_matches!(result_with_brackets, Err(ChatLinkError::InvalidString));
    }

    #[test]
    fn test_parse_invalid_base64() {
        let raw = "hello";
        
        let result = ChatLink::try_from(raw);
        assert_matches!(result, Err(ChatLinkError::InvalidBase64(_)));

        let raw_with_brackets = format!("[&{}]", raw);
        let result_with_brackets = ChatLink::try_from(raw_with_brackets.as_str());
        assert_matches!(result_with_brackets, Err(ChatLinkError::InvalidBase64(_)));
    }

    #[test]
    fn test_parse_invalid_link_type() {
        let raw = "abcd";
        
        let result = ChatLink::try_from(raw);
        assert_matches!(result, Err(ChatLinkError::UnknownType(_)));

        let raw_with_brackets = format!("[&{}]", raw);
        let result_with_brackets = ChatLink::try_from(raw_with_brackets.as_str());
        assert_matches!(result_with_brackets, Err(ChatLinkError::UnknownType(_)));
    }

    #[test]
    fn test_parse_invalid_length() {
        let raw = "DwAAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAAAAAAAAAD/fw==";
        
        let result = ChatLink::try_from(raw);
        assert_matches!(result, Err(ChatLinkError::TruncatedData(_)));

        let raw_with_brackets = format!("[&{}]", raw);
        let result_with_brackets = ChatLink::try_from(raw_with_brackets.as_str());
        assert_matches!(result_with_brackets, Err(ChatLinkError::TruncatedData(_)));
    }

    #[test]
    fn test_parse_empty() {
        let raw = EMPTY_TEMPLATE;
        let expected_slots = SlotType::iter().map(|slot_type| {
            (slot_type, empty_skin(slot_type))
        }).collect();
        let expected_template = WardrobeTemplate::new(expected_slots);
        
        let result = &ChatLink::try_from(raw).unwrap();
        assert_matches!(result, ChatLink::WardrobeTemplate(actual) if actual == &expected_template);

        let raw_with_brackets = format!("[&{}]", raw);
        let result_with_brackets = ChatLink::try_from(raw_with_brackets.as_str());

        assert_matches!(&result_with_brackets, Ok(ChatLink::WardrobeTemplate(actual)) if actual == &expected_template);

        let actual_encoded: String = result.try_into().unwrap();
        assert_eq!(actual_encoded, raw);
    }

    #[test]
    fn test_parse_zizi() {
        let raw = ZIZI_TEMPLATE;
        let result = &ChatLink::try_from(raw).unwrap();

        let ChatLink::WardrobeTemplate(actual) = result else {
           panic!("Expected WardrobeTemplate, got {result:?}");
        };

        for (slot_type, slot) in actual {
            match slot {
                EquipmentSlot::NonDyable{ skin, visible } => {
                    match slot_type {
                        SlotType::WeaponB2 => {
                            assert_eq!(skin, &0.into(), "Expected unset skin_id for {skin:?}");
                            assert!(visible, "Expected skin to be visible for {skin:?}");
                        },
                        SlotType::Aquabreather => {
                            assert_ne!(skin, &0.into(), "Expected skin_id set for {skin:?}");
                            assert!(!visible, "Expected skin to not be visible for {skin:?}");
                        },
                        SlotType::WeaponAquaticA | SlotType::WeaponAquaticB | SlotType::WeaponA1 | SlotType::WeaponA2 | SlotType::WeaponB1 => {
                            assert_ne!(skin, &0.into(), "Expected skin_id set for {skin:?}");
                            assert!(visible, "Expected skin to be visible for {skin:?}");
                        },
                        _ => panic!("Dyable skin should not be non-dyable {skin:?}")
                    }
                }
                EquipmentSlot::Dyable{ skin, visible, dyes } => {
                    match slot_type {
                        SlotType::Outfit => {
                            assert_ne!(skin, &0.into(), "Expected skin_id set for {skin:?}");
                            assert!(!visible, "Expected skin to not be visible for {skin:?}");
                            assert_eq!(dyes, &(1, 1, 1, 1).into(), "Expected unset dyes for {skin:?}");
                        },
                        SlotType::Backpack => {
                            assert_ne!(skin, &0.into(), "Expected skin_id set for {skin:?}");
                            assert!(visible, "Expected skin to be visible for {skin:?}");
                            assert_eq!(dyes, &(1, 1, 1, 1).into(), "Expected unset dyes for {skin:?}");
                        },
                        SlotType::Chest | SlotType::Shoes | SlotType::Gloves | SlotType::Head | SlotType::Legs | SlotType::Shoulders => {
                            assert_ne!(skin, &0.into(), "Expected skin_id set for {skin:?}");
                            assert!(visible, "Expected skin to be visible for {skin:?}");
                            assert_ne!(dyes, &(1, 1, 1, 1).into(), "Expected dyes set for {skin:?}");
                        },
                        _ => panic!("Skin should not be dyable {skin:?}")
                    }
                }
            }
        }

        let actual_encoded: String = result.try_into().unwrap();
        assert_eq!(actual_encoded, raw);
    }

    #[test]
    fn test_parse_zizi_armor_only() {
        let raw = ZIZI_ARMOR_TEMPLATE;
        let result = &ChatLink::try_from(raw).unwrap();

        let ChatLink::WardrobeTemplate(actual) = result else {
           panic!("Expected WardrobeTemplate, got {result:?}");
        };

        for (slot_type, slot) in actual {
            match slot {
                EquipmentSlot::NonDyable{ skin, visible } => {
                    match slot_type {
                        SlotType::WeaponAquaticA | SlotType::WeaponAquaticB | SlotType::WeaponA1 | SlotType::WeaponA2 | SlotType::WeaponB1 | SlotType::WeaponB2 | SlotType::Aquabreather => {
                            assert_eq!(skin, &0.into(), "Expected unset skin_id for {skin:?}");
                            assert!(visible, "Expected skin to be visible for {skin:?}");
                        },
                        _ => panic!("Dyable skin should not be non-dyable {skin:?}")
                    }
                }
                EquipmentSlot::Dyable{ skin, visible, dyes } => {
                    match slot_type {
                        SlotType::Outfit => {
                            assert_eq!(skin, &0.into(), "Expected unset skin_id for {skin:?}");
                            assert!(visible, "Expected skin to be visible for {skin:?}");
                            assert_eq!(dyes, &(1, 1, 1, 1).into(), "Expected unset dyes for {skin:?}");
                        },
                        SlotType::Backpack => {
                            assert_ne!(skin, &0.into(), "Expected skin_id set for {skin:?}");
                            assert!(visible, "Expected skin to be visible for {skin:?}");
                            assert_eq!(dyes, &(1, 1, 1, 1).into(), "Expected unset dyes for {skin:?}");
                        },
                        SlotType::Chest | SlotType::Shoes | SlotType::Gloves | SlotType::Head | SlotType::Legs | SlotType::Shoulders => {
                            assert_ne!(skin, &0.into(), "Expected skin_id set for {skin:?}");
                            assert!(visible, "Expected skin to be visible for {skin:?}");
                            assert_ne!(dyes, &(1, 1, 1, 1).into(), "Expected dyes set for {skin:?}");
                        },
                        _ => panic!("Skin should not be dyable {skin:?}")
                    }
                }
            }
        }

        let actual_encoded: String = result.try_into().unwrap();
        assert_eq!(actual_encoded, raw);
    }

    fn empty_skin(slot_type: SlotType) -> EquipmentSlot {
        if slot_type.dyable() {
            EquipmentSlot::Dyable{ skin: 0.into(), visible: true, dyes: (1, 1, 1, 1).into() }
        } else {
            EquipmentSlot::NonDyable { skin: 0.into(), visible: true }
        }
    }
}
