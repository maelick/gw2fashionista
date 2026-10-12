use gw2fashionista_chatlink::fixtures::travel::{EMPTY_TEMPLATE, PEEKABOO_TEMPLATE, ZIZI_TEMPLATE};
use gw2fashionista_chatlink::{
    skins::{Appearance, Dyes, SkinId},
    templates::travel::{TravelSlot, TravelTemplate},
};
use linearize::LinearizeExt;

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
}

#[test]
#[test_log::test]
fn test_parse_peekaboo() {
    let actual = &PEEKABOO_TEMPLATE.link.parse::<TravelTemplate>().unwrap();

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
                        assert_ne!(dyes, &Dyes::default(), "Expected dyes set for {skin:?}");
                    }
                }
            }
            _ => panic!("Travel template slot {slot:?} should be dyeable"),
        }
    }
}

#[test]
#[test_log::test]
fn test_parse_zizi() {
    let actual = &ZIZI_TEMPLATE.link.parse::<TravelTemplate>().unwrap();

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
                        assert_ne!(dyes, &Dyes::default(), "Expected dyes set for {skin:?}");
                    }
                }
            }
            _ => panic!("Travel template slot {slot:?} should be dyeable"),
        }
    }
}

fn empty_skin() -> Appearance {
    Appearance::Dyeable {
        skin: SkinId::default(),
        visible: true,
        dyes: Dyes::default(),
    }
}
