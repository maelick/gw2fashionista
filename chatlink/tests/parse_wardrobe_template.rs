use gw2fashionista_chatlink::fixtures::wardrobe::{
    EMPTY_TEMPLATE, ZIZI_ARMOR_TEMPLATE, ZIZI_TEMPLATE,
};
use gw2fashionista_chatlink::{
    skins::{Appearance, Dyes, SkinId},
    templates::{
        FashionSlot,
        wardrobe::{WardrobeSlot, WardrobeTemplate},
    },
};
use linearize::LinearizeExt;

#[test]
#[test_log::test]
fn test_parse_empty() {
    let expected_slots = WardrobeSlot::variants()
        .map(|slot| (slot, empty_skin(slot)))
        .collect();
    let expected_template = WardrobeTemplate::new(expected_slots);
    let actual = &EMPTY_TEMPLATE.link.parse::<WardrobeTemplate>().unwrap();

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
fn test_parse_zizi() {
    let actual = &ZIZI_TEMPLATE.link.parse::<WardrobeTemplate>().unwrap();

    for (slot, appearance) in actual {
        match appearance {
            Appearance::NonDyeable { skin, visible } => match slot {
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
                _ => panic!("Dyeable skin should not be non-dyeable {skin:?}"),
            },
            Appearance::Dyeable {
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
                            &Dyes::default(),
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
                            &Dyes::default(),
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
                        assert_ne!(dyes, &Dyes::default(), "Expected dyes set for {skin:?}");
                    }
                    _ => panic!("Skin should not be dyeable {skin:?}"),
                }
            }
        }
    }
}

#[test]
#[test_log::test]
fn test_parse_zizi_armor_only() {
    let actual = &ZIZI_ARMOR_TEMPLATE
        .link
        .parse::<WardrobeTemplate>()
        .unwrap();

    for (slot, appearance) in actual {
        match appearance {
            Appearance::NonDyeable { skin, visible } => match slot {
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
                _ => panic!("Dyeable skin should not be non-dyeable {appearance:?}"),
            },
            Appearance::Dyeable {
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
                        &Dyes::default(),
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
                        &Dyes::default(),
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
                    assert_ne!(dyes, &Dyes::default(), "Expected dyes set for {skin:?}");
                    assert!(!appearance.is_empty(), "Expect not empty slot {slot:?}");
                }
                _ => panic!("Skin should not be dyeable {skin:?}"),
            },
        }
    }
}

fn empty_skin(slot: WardrobeSlot) -> Appearance {
    if slot.dyeable() {
        Appearance::Dyeable {
            skin: SkinId::default(),
            visible: true,
            dyes: Dyes::default(),
        }
    } else {
        Appearance::NonDyeable {
            skin: SkinId::default(),
            visible: true,
        }
    }
}
