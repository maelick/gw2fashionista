#[cfg(test)]
mod tests {
    use gw2fashionista_core::domain::{
        chatlink::ChatLink,
        error::ChatLinkError,
        skins::{Appearance, Dyes, SkinId},
        templates::travel::{TravelSlot, TravelTemplate},
    };
    use linearize::LinearizeExt;
    use std::assert_matches;

    use gw2fashionista_fixtures::travel::{EMPTY_TEMPLATE, PEEKABOO_TEMPLATE, ZIZI_TEMPLATE};

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
        let raw = "EAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEBAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAP8P";

        let result = ChatLink::try_from(raw);
        assert_matches!(result, Err(ChatLinkError::TruncatedData(_)));

        let raw_with_brackets = format!("[&{}]", raw);
        let result_with_brackets = ChatLink::try_from(raw_with_brackets.as_str());
        assert_matches!(result_with_brackets, Err(ChatLinkError::TruncatedData(_)));
    }

    #[test]
    fn test_parse_empty() {
        let raw = EMPTY_TEMPLATE.chat_link;
        let expected_slots = TravelSlot::variants()
            .map(|slot| (slot, empty_skin()))
            .collect();
        let expected_template = TravelTemplate::new(expected_slots);

        let result = &ChatLink::try_from(raw).unwrap();
        let ChatLink::TravelTemplate(actual) = result else {
            panic!("Expected TravelTemplate, got {result:?}");
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

        assert_matches!(result_with_brackets, ChatLink::TravelTemplate(actual) if actual == &expected_template);

        let actual_encoded: String = result_with_brackets.try_into().unwrap();
        assert_eq!(actual_encoded, raw_with_brackets);
    }

    #[test]
    fn test_parse_peekaboo() {
        let raw = format!("[&{}]", PEEKABOO_TEMPLATE.chat_link);
        let result = &ChatLink::try_from(raw.as_str()).unwrap();

        let ChatLink::TravelTemplate(actual) = result else {
            panic!("Expected TravelTemplate, got {result:?}");
        };

        for (slot, appearance) in actual {
            match appearance {
                Appearance::Dyeable {
                    skin,
                    visible,
                    dyes,
                } => {
                    assert!(visible, "Expected skin to be visible for {skin:?}");
                    assert!(!appearance.is_empty(), "Expect not empty slot {slot:?}");
                    match slot {
                        TravelSlot::Glider
                        | TravelSlot::Doorway
                        | TravelSlot::Jackal
                        | TravelSlot::Griffon
                        | TravelSlot::Springer
                        | TravelSlot::Skimmer
                        | TravelSlot::Raptor
                        | TravelSlot::Beetle
                        | TravelSlot::Warclaw
                        | TravelSlot::Skyscale
                        | TravelSlot::Skiff
                        | TravelSlot::Turtle => {
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
                    }
                }
                _ => panic!("Travel template slot {slot:?} should be dyeable"),
            }
        }

        let actual_encoded: String = result.try_into().unwrap();
        assert_eq!(actual_encoded, raw);
    }

    #[test]
    fn test_parse_zizi() {
        let raw = format!("[&{}]", ZIZI_TEMPLATE.chat_link);
        let result = &ChatLink::try_from(raw.as_str()).unwrap();

        let ChatLink::TravelTemplate(actual) = result else {
            panic!("Expected TravelTemplate, got {result:?}");
        };

        for (slot, appearance) in actual {
            match appearance {
                Appearance::Dyeable {
                    skin,
                    visible,
                    dyes,
                } => {
                    assert!(visible, "Expected skin to be visible for {skin:?}");
                    assert!(!appearance.is_empty(), "Expect not empty slot {slot:?}");
                    match slot {
                        TravelSlot::Glider
                        | TravelSlot::Doorway
                        | TravelSlot::Jackal
                        | TravelSlot::Griffon
                        | TravelSlot::Springer
                        | TravelSlot::Skimmer
                        | TravelSlot::Raptor
                        | TravelSlot::Beetle
                        | TravelSlot::Warclaw
                        | TravelSlot::Skyscale
                        | TravelSlot::Skiff
                        | TravelSlot::Turtle => {
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
                    }
                }
                _ => panic!("Travel template slot {slot:?} should be dyeable"),
            }
        }

        let actual_encoded: String = result.try_into().unwrap();
        assert_eq!(actual_encoded, raw);
    }

    fn empty_skin() -> Appearance {
        Appearance::Dyeable {
            skin: SkinId::default(),
            visible: true,
            dyes: Dyes::default(),
        }
    }
}
