#[cfg(test)]
mod tests {
    use gw2fashionista_core::domain::{
        chatlink::ChatLink,
        error::ChatLinkError,
        skins::{Appearance, Dyes, SkinId},
        templates::{
            FashionSlot,
            wardrobe::{WardrobeTemplate, slot::WardrobeSlot},
        },
    };
    use linearize::LinearizeExt;
    use std::assert_matches;

    use gw2fashionista_fixtures::wardrobe::{EMPTY_TEMPLATE, ZIZI_ARMOR_TEMPLATE, ZIZI_TEMPLATE};

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
        let raw = EMPTY_TEMPLATE.chat_link;
        let expected_slots = WardrobeSlot::variants()
            .map(|slot| (slot, empty_skin(slot)))
            .collect();
        let expected_template = WardrobeTemplate::new(expected_slots);

        let result = &ChatLink::try_from(raw).unwrap();
        let ChatLink::WardrobeTemplate(actual) = result else {
            panic!("Expected WardrobeTemplate, got {result:?}");
        };

        assert_eq!(actual, &expected_template);
        for (slot, appearance) in actual {
            assert!(
                appearance.is_empty(),
                "Expect empty slot {slot:?} but got {appearance:?}"
            );
        }

        let raw_with_brackets = format!("[&{}]", raw);
        let result_with_brackets = &ChatLink::try_from(raw_with_brackets.as_str()).unwrap();

        assert_matches!(result_with_brackets, ChatLink::WardrobeTemplate(actual) if actual == &expected_template);

        let actual_encoded: String = result_with_brackets.try_into().unwrap();
        assert_eq!(actual_encoded, raw_with_brackets);
    }

    #[test]
    fn test_parse_zizi() {
        let raw = format!("[&{}]", ZIZI_TEMPLATE.chat_link);
        let result = &ChatLink::try_from(raw.as_str()).unwrap();

        let ChatLink::WardrobeTemplate(actual) = result else {
            panic!("Expected WardrobeTemplate, got {result:?}");
        };

        for (slot, appearance) in actual {
            match appearance {
                Appearance::NonDyable { skin, visible } => match slot {
                    WardrobeSlot::WeaponB2 => {
                        assert_eq!(
                            skin,
                            &SkinId::default(),
                            "Expected unset skin_id for {skin:?}"
                        );
                        assert!(visible, "Expected skin to be visible for {skin:?}");
                        assert!(
                            appearance.is_empty(),
                            "Expect empty slot {slot:?} but got {appearance:?}"
                        );
                    }
                    WardrobeSlot::Aquabreather => {
                        assert_ne!(
                            skin,
                            &SkinId::default(),
                            "Expected skin_id set for {skin:?}"
                        );
                        assert!(!visible, "Expected skin to not be visible for {skin:?}");
                        assert!(!appearance.is_empty(), "Expect not empty slot {slot:?}");
                    }
                    WardrobeSlot::WeaponAquaticA
                    | WardrobeSlot::WeaponAquaticB
                    | WardrobeSlot::WeaponA1
                    | WardrobeSlot::WeaponA2
                    | WardrobeSlot::WeaponB1 => {
                        assert_ne!(
                            skin,
                            &SkinId::default(),
                            "Expected skin_id set for {skin:?}"
                        );
                        assert!(visible, "Expected skin to be visible for {skin:?}");
                        assert!(!appearance.is_empty(), "Expect not empty slot {slot:?}");
                    }
                    _ => panic!("Dyable skin should not be non-dyable {skin:?}"),
                },
                Appearance::Dyable {
                    skin,
                    visible,
                    dyes,
                } => {
                    assert!(!appearance.is_empty(), "Expect not empty slot {slot:?}");
                    match slot {
                        WardrobeSlot::Outfit => {
                            assert_ne!(
                                skin,
                                &SkinId::default(),
                                "Expected skin_id set for {skin:?}"
                            );
                            assert!(!visible, "Expected skin to not be visible for {skin:?}");
                            assert_eq!(
                                dyes,
                                &(1, 1, 1, 1).into(),
                                "Expected unset dyes for {skin:?}"
                            );
                        }
                        WardrobeSlot::Backpack => {
                            assert_ne!(
                                skin,
                                &SkinId::default(),
                                "Expected skin_id set for {skin:?}"
                            );
                            assert!(visible, "Expected skin to be visible for {skin:?}");
                            assert_eq!(
                                dyes,
                                &(1, 1, 1, 1).into(),
                                "Expected unset dyes for {skin:?}"
                            );
                        }
                        WardrobeSlot::Chest
                        | WardrobeSlot::Shoes
                        | WardrobeSlot::Gloves
                        | WardrobeSlot::Head
                        | WardrobeSlot::Legs
                        | WardrobeSlot::Shoulders => {
                            assert_ne!(
                                skin,
                                &SkinId::default(),
                                "Expected skin_id set for {skin:?}"
                            );
                            assert!(visible, "Expected skin to be visible for {skin:?}");
                            assert_ne!(
                                dyes,
                                &(1, 1, 1, 1).into(),
                                "Expected dyes set for {skin:?}"
                            );
                        }
                        _ => panic!("Skin should not be dyable {skin:?}"),
                    }
                }
            }
        }

        let actual_encoded: String = result.try_into().unwrap();
        assert_eq!(actual_encoded, raw);
    }

    #[test]
    fn test_parse_zizi_armor_only() {
        let raw = format!("[&{}]", ZIZI_ARMOR_TEMPLATE.chat_link);
        let result = &ChatLink::try_from(raw.as_str()).unwrap();

        let ChatLink::WardrobeTemplate(actual) = result else {
            panic!("Expected WardrobeTemplate, got {result:?}");
        };

        for (slot, appearance) in actual {
            match appearance {
                Appearance::NonDyable { skin, visible } => match slot {
                    WardrobeSlot::WeaponAquaticA
                    | WardrobeSlot::WeaponAquaticB
                    | WardrobeSlot::WeaponA1
                    | WardrobeSlot::WeaponA2
                    | WardrobeSlot::WeaponB1
                    | WardrobeSlot::WeaponB2
                    | WardrobeSlot::Aquabreather => {
                        assert_eq!(
                            skin,
                            &SkinId::default(),
                            "Expected unset skin_id for {skin:?}"
                        );
                        assert!(visible, "Expected skin to be visible for {skin:?}");
                        assert!(
                            appearance.is_empty(),
                            "Expect empty slot {slot:?} but got {appearance:?}"
                        );
                    }
                    _ => panic!("Dyable skin should not be non-dyable {appearance:?}"),
                },
                Appearance::Dyable {
                    skin,
                    visible,
                    dyes,
                } => match slot {
                    WardrobeSlot::Outfit => {
                        assert_eq!(
                            skin,
                            &SkinId::default(),
                            "Expected unset skin_id for {skin:?}"
                        );
                        assert!(visible, "Expected skin to be visible for {skin:?}");
                        assert_eq!(
                            dyes,
                            &(1, 1, 1, 1).into(),
                            "Expected unset dyes for {skin:?}"
                        );
                        assert!(
                            appearance.is_empty(),
                            "Expect empty slot {slot:?} but got {appearance:?}"
                        );
                    }
                    WardrobeSlot::Backpack => {
                        assert_ne!(
                            skin,
                            &SkinId::default(),
                            "Expected skin_id set for {skin:?}"
                        );
                        assert!(visible, "Expected skin to be visible for {skin:?}");
                        assert_eq!(
                            dyes,
                            &(1, 1, 1, 1).into(),
                            "Expected unset dyes for {skin:?}"
                        );
                        assert!(!appearance.is_empty(), "Expect not empty slot {slot:?}");
                    }
                    WardrobeSlot::Chest
                    | WardrobeSlot::Shoes
                    | WardrobeSlot::Gloves
                    | WardrobeSlot::Head
                    | WardrobeSlot::Legs
                    | WardrobeSlot::Shoulders => {
                        assert_ne!(
                            skin,
                            &SkinId::default(),
                            "Expected skin_id set for {skin:?}"
                        );
                        assert!(visible, "Expected skin to be visible for {skin:?}");
                        assert_ne!(dyes, &(1, 1, 1, 1).into(), "Expected dyes set for {skin:?}");
                        assert!(!appearance.is_empty(), "Expect not empty slot {slot:?}");
                    }
                    _ => panic!("Skin should not be dyable {skin:?}"),
                },
            }
        }

        let actual_encoded: String = result.try_into().unwrap();
        assert_eq!(actual_encoded, raw);
    }

    fn empty_skin(slot: WardrobeSlot) -> Appearance {
        if slot.dyable() {
            Appearance::Dyable {
                skin: SkinId::default(),
                visible: true,
                dyes: Dyes::default(),
            }
        } else {
            Appearance::NonDyable {
                skin: SkinId::default(),
                visible: true,
            }
        }
    }
}
