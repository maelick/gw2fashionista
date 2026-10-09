use gw2fashionista_chatlink::fixtures::travel::{EMPTY_TEMPLATE, PEEKABOO_TEMPLATE, ZIZI_TEMPLATE};
use gw2fashionista_chatlink::{
    ChatLink,
    skins::{Appearance, Dyes, SkinId},
    templates::travel::{TravelSlot, TravelTemplate},
};
use linearize::LinearizeExt;
use std::assert_matches;

#[test]
#[test_log::test]
fn test_parse_empty() {
    let expected_slots = TravelSlot::variants()
        .map(|slot| (slot, empty_skin()))
        .collect();
    let expected_template = TravelTemplate::new(expected_slots);
    let actual = &EMPTY_TEMPLATE.link.parse::<TravelTemplate>().unwrap();

    assert_eq!(actual, &expected_template);
    for (slot, appearance) in actual {
        assert!(
            appearance.is_empty(),
            "Expect empty slot {slot:?} but got {appearance:?}"
        );
    }

    let result_with_brackets = &EMPTY_TEMPLATE.wrap().parse().unwrap();

    assert_matches!(result_with_brackets, ChatLink::TravelTemplate(actual) if actual == &expected_template);
    assert_eq!(result_with_brackets.to_string(), EMPTY_TEMPLATE.wrap());
}

#[test]
#[test_log::test]
fn test_parse_peekaboo() {
    let result = &PEEKABOO_TEMPLATE.link.parse().unwrap();

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
                        assert_ne!(dyes, &(1, 1, 1, 1).into(), "Expected dyes set for {skin:?}");
                    }
                }
            }
            _ => panic!("Travel template slot {slot:?} should be dyeable"),
        }
    }

    assert_eq!(result.to_string(), PEEKABOO_TEMPLATE.wrap());
}

#[test]
#[test_log::test]
fn test_parse_zizi() {
    let result = &ZIZI_TEMPLATE.link.parse().unwrap();

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
                        assert_ne!(dyes, &(1, 1, 1, 1).into(), "Expected dyes set for {skin:?}");
                    }
                }
            }
            _ => panic!("Travel template slot {slot:?} should be dyeable"),
        }
    }

    assert_eq!(result.to_string(), ZIZI_TEMPLATE.wrap());
}

fn empty_skin() -> Appearance {
    Appearance::Dyeable {
        skin: SkinId::default(),
        visible: true,
        dyes: Dyes::default(),
    }
}
