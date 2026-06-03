#[cfg(test)]
mod tests {
    use gw2fashionista_core::domain::{chatlink::ChatLink, error::ChatLinkError, fashion_template::{DyableSkin, EquipmentSlot, FashionTemplate, Skin}, skin_type::SkinType};
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
        let expected_slots = SkinType::iter().map(|skin_type| {
            (skin_type, empty_skin(skin_type))
        }).collect();
        let expected_template = FashionTemplate::new(expected_slots);
        
        let result = ChatLink::try_from(raw);
        assert_matches!(result, Ok(ChatLink::WardrobeTemplate(actual)) if actual == expected_template);

        let raw_with_brackets = format!("[&{}]", raw);
        let result_with_brackets = ChatLink::try_from(raw_with_brackets.as_str());

        assert_matches!(result_with_brackets, Ok(ChatLink::WardrobeTemplate(actual)) if actual == expected_template);
    }

    #[test]
    fn test_parse_zizi() {
        let raw = ZIZI_TEMPLATE;
        let result = ChatLink::try_from(raw);

        let Ok(ChatLink::WardrobeTemplate(actual)) = result else {
           panic!("Expected WardrobeTemplate, got {result:?}");
        };

        for slot in actual {
            match slot {
                EquipmentSlot::Skin(skin) => {
                    match skin.skin_type {
                        SkinType::WeaponB2 => {
                            assert_eq!(skin.skin, 0.into(), "Expected unset skin_id for {skin:?}");
                            assert!(skin.visible, "Expected skin to be visible for {skin:?}");
                        },
                        SkinType::Aquabreather => {
                            assert_ne!(skin.skin, 0.into(), "Expected skin_id set for {skin:?}");
                            assert!(!skin.visible, "Expected skin to not be visible for {skin:?}");
                        },
                        SkinType::WeaponAquaticA | SkinType::WeaponAquaticB | SkinType::WeaponA1 | SkinType::WeaponA2 | SkinType::WeaponB1 => {
                            assert_ne!(skin.skin, 0.into(), "Expected skin_id set for {skin:?}");
                            assert!(skin.visible, "Expected skin to be visible for {skin:?}");
                        },
                        _ => panic!("Dyable skin should not be non-dyable {skin:?}")
                    }
                }
                EquipmentSlot::DyableSkin(skin) => {
                    match skin.skin.skin_type {
                        SkinType::Outfit => {
                            assert_ne!(skin.skin.skin, 0.into(), "Expected skin_id set for {skin:?}");
                            assert!(!skin.skin.visible, "Expected skin to not be visible for {skin:?}");
                            assert_eq!(skin.dyes, (1, 1, 1, 1).into(), "Expected unset dyes for {skin:?}");
                        },
                        SkinType::Backpack => {
                            assert_ne!(skin.skin.skin, 0.into(), "Expected skin_id set for {skin:?}");
                            assert!(skin.skin.visible, "Expected skin to be visible for {skin:?}");
                            assert_eq!(skin.dyes, (1, 1, 1, 1).into(), "Expected unset dyes for {skin:?}");
                        },
                        SkinType::Chest | SkinType::Shoes | SkinType::Gloves | SkinType::Head | SkinType::Legs | SkinType::Shoulders => {
                            assert_ne!(skin.skin.skin, 0.into(), "Expected skin_id set for {skin:?}");
                            assert!(skin.skin.visible, "Expected skin to be visible for {skin:?}");
                            assert_ne!(skin.dyes, (1, 1, 1, 1).into(), "Expected dyes set for {skin:?}");
                        },
                        _ => panic!("Skin should not be dyable {skin:?}")
                    }
                }
            }
        }
    }

    #[test]
    fn test_parse_zizi_armor_only() {
        let raw = ZIZI_ARMOR_TEMPLATE;
        let result = ChatLink::try_from(raw);

        let Ok(ChatLink::WardrobeTemplate(actual)) = result else {
           panic!("Expected WardrobeTemplate, got {result:?}");
        };

        for slot in actual {
            match slot {
                EquipmentSlot::Skin(skin) => {
                    match skin.skin_type {
                        SkinType::WeaponAquaticA | SkinType::WeaponAquaticB | SkinType::WeaponA1 | SkinType::WeaponA2 | SkinType::WeaponB1 | SkinType::WeaponB2 | SkinType::Aquabreather => {
                            assert_eq!(skin.skin, 0.into(), "Expected unset skin_id for {skin:?}");
                            assert!(skin.visible, "Expected skin to be visible for {skin:?}");
                        },
                        _ => panic!("Dyable skin should not be non-dyable {skin:?}")
                    }
                }
                EquipmentSlot::DyableSkin(skin) => {
                    match skin.skin.skin_type {
                        SkinType::Outfit => {
                            assert_eq!(skin.skin.skin, 0.into(), "Expected unset skin_id for {skin:?}");
                            assert!(skin.skin.visible, "Expected skin to be visible for {skin:?}");
                            assert_eq!(skin.dyes, (1, 1, 1, 1).into(), "Expected unset dyes for {skin:?}");
                        },
                        SkinType::Backpack => {
                            assert_ne!(skin.skin.skin, 0.into(), "Expected skin_id set for {skin:?}");
                            assert!(skin.skin.visible, "Expected skin to be visible for {skin:?}");
                            assert_eq!(skin.dyes, (1, 1, 1, 1).into(), "Expected unset dyes for {skin:?}");
                        },
                        SkinType::Chest | SkinType::Shoes | SkinType::Gloves | SkinType::Head | SkinType::Legs | SkinType::Shoulders => {
                            assert_ne!(skin.skin.skin, 0.into(), "Expected skin_id set for {skin:?}");
                            assert!(skin.skin.visible, "Expected skin to be visible for {skin:?}");
                            assert_ne!(skin.dyes, (1, 1, 1, 1).into(), "Expected dyes set for {skin:?}");
                        },
                        _ => panic!("Skin should not be dyable {skin:?}")
                    }
                }
            }
        }
    }

    fn empty_skin(skin_type: SkinType) -> EquipmentSlot {
        let skin = Skin::new(skin_type, 0.into(), true);
        if skin_type.dyable() {
            DyableSkin::new(skin, (1, 1, 1, 1).into()).into()
        } else {
            skin.into()
        }
    }
}
