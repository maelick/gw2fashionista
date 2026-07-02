#[cfg(test)]
mod tests {
    use gw2fashionista_core::domain::{
        chatlink::ChatLink,
        error::ChatLinkError,
        skins::{Dyes, SkinId},
        wardrobe_template::{
            WardrobeTemplate,
            slot::{SlotType, WardrobeSlot},
        },
    };
    use std::assert_matches;
    use strum::IntoEnumIterator;

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
        let expected_slots = SlotType::iter()
            .map(|slot_type| (slot_type, empty_skin(slot_type)))
            .collect();
        let expected_template = WardrobeTemplate::new(expected_slots);

        let result = &ChatLink::try_from(raw).unwrap();
        let ChatLink::WardrobeTemplate(actual) = result else {
            panic!("Expected WardrobeTemplate, got {result:?}");
        };

        assert_eq!(actual, &expected_template);
        for (slot_type, slot) in actual {
            assert!(
                slot.is_empty(),
                "Expect empty slot {slot_type:?} but got {slot:?}"
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

        for (slot_type, slot) in actual {
            match slot {
                WardrobeSlot::NonDyable { skin, visible } => match slot_type {
                    SlotType::WeaponB2 => {
                        assert_eq!(
                            skin,
                            &SkinId::default(),
                            "Expected unset skin_id for {skin:?}"
                        );
                        assert!(visible, "Expected skin to be visible for {skin:?}");
                        assert!(
                            slot.is_empty(),
                            "Expect empty slot {slot_type:?} but got {slot:?}"
                        );
                    }
                    SlotType::Aquabreather => {
                        assert_ne!(
                            skin,
                            &SkinId::default(),
                            "Expected skin_id set for {skin:?}"
                        );
                        assert!(!visible, "Expected skin to not be visible for {skin:?}");
                        assert!(!slot.is_empty(), "Expect not empty slot {slot_type:?}");
                    }
                    SlotType::WeaponAquaticA
                    | SlotType::WeaponAquaticB
                    | SlotType::WeaponA1
                    | SlotType::WeaponA2
                    | SlotType::WeaponB1 => {
                        assert_ne!(
                            skin,
                            &SkinId::default(),
                            "Expected skin_id set for {skin:?}"
                        );
                        assert!(visible, "Expected skin to be visible for {skin:?}");
                        assert!(!slot.is_empty(), "Expect not empty slot {slot_type:?}");
                    }
                    _ => panic!("Dyable skin should not be non-dyable {skin:?}"),
                },
                WardrobeSlot::Dyable {
                    skin,
                    visible,
                    dyes,
                } => {
                    assert!(!slot.is_empty(), "Expect not empty slot {slot_type:?}");
                    match slot_type {
                        SlotType::Outfit => {
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
                        SlotType::Backpack => {
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
                        SlotType::Chest
                        | SlotType::Shoes
                        | SlotType::Gloves
                        | SlotType::Head
                        | SlotType::Legs
                        | SlotType::Shoulders => {
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

        for (slot_type, slot) in actual {
            match slot {
                WardrobeSlot::NonDyable { skin, visible } => match slot_type {
                    SlotType::WeaponAquaticA
                    | SlotType::WeaponAquaticB
                    | SlotType::WeaponA1
                    | SlotType::WeaponA2
                    | SlotType::WeaponB1
                    | SlotType::WeaponB2
                    | SlotType::Aquabreather => {
                        assert_eq!(
                            skin,
                            &SkinId::default(),
                            "Expected unset skin_id for {skin:?}"
                        );
                        assert!(visible, "Expected skin to be visible for {skin:?}");
                        assert!(
                            slot.is_empty(),
                            "Expect empty slot {slot_type:?} but got {slot:?}"
                        );
                    }
                    _ => panic!("Dyable skin should not be non-dyable {slot:?}"),
                },
                WardrobeSlot::Dyable {
                    skin,
                    visible,
                    dyes,
                } => match slot_type {
                    SlotType::Outfit => {
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
                            slot.is_empty(),
                            "Expect empty slot {slot_type:?} but got {slot:?}"
                        );
                    }
                    SlotType::Backpack => {
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
                        assert!(!slot.is_empty(), "Expect not empty slot {slot_type:?}");
                    }
                    SlotType::Chest
                    | SlotType::Shoes
                    | SlotType::Gloves
                    | SlotType::Head
                    | SlotType::Legs
                    | SlotType::Shoulders => {
                        assert_ne!(
                            skin,
                            &SkinId::default(),
                            "Expected skin_id set for {skin:?}"
                        );
                        assert!(visible, "Expected skin to be visible for {skin:?}");
                        assert_ne!(dyes, &(1, 1, 1, 1).into(), "Expected dyes set for {skin:?}");
                        assert!(!slot.is_empty(), "Expect not empty slot {slot_type:?}");
                    }
                    _ => panic!("Skin should not be dyable {skin:?}"),
                },
            }
        }

        let actual_encoded: String = result.try_into().unwrap();
        assert_eq!(actual_encoded, raw);
    }

    fn empty_skin(slot_type: SlotType) -> WardrobeSlot {
        if slot_type.dyable() {
            WardrobeSlot::Dyable {
                skin: SkinId::default(),
                visible: true,
                dyes: Dyes::default(),
            }
        } else {
            WardrobeSlot::NonDyable {
                skin: SkinId::default(),
                visible: true,
            }
        }
    }
}
