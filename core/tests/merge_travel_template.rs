use gw2fashionista_core::domain::{
    chatlink::ChatLink,
    templates::{FashionSlot, FashionSlotKind},
};
use gw2fashionista_fixtures::travel::{KABOOM_MOUNTS_TEMPLATE, PEEKABOO_TEMPLATE};

#[test]
#[test_log::test]
fn test_merge_peekaboo_with_kaboom_mounts() {
    let chat_link = &ChatLink::try_from(PEEKABOO_TEMPLATE.chat_link).unwrap();
    let ChatLink::TravelTemplate(base_template) = chat_link else {
        panic!("Expected TravelTemplate, got {chat_link:?}");
    };

    let chat_link = &ChatLink::try_from(KABOOM_MOUNTS_TEMPLATE.chat_link).unwrap();
    let ChatLink::TravelTemplate(mount_template) = chat_link else {
        panic!("Expected TravelTemplate, got {chat_link:?}");
    };

    let merged = base_template.merge(&mount_template, false, false);

    for (slot, appearance) in &merged {
        if slot.kind() == FashionSlotKind::Mount {
            assert_eq!(appearance, mount_template.get_slot(&slot));
        } else {
            assert_eq!(appearance, base_template.get_slot(&slot));
        }
    }
}

#[test]
#[test_log::test]
fn test_merge_peekaboo_with_kaboom_mounts_skins_only() {
    let chat_link = &ChatLink::try_from(PEEKABOO_TEMPLATE.chat_link).unwrap();
    let ChatLink::TravelTemplate(base_template) = chat_link else {
        panic!("Expected TravelTemplate, got {chat_link:?}");
    };

    let chat_link = &ChatLink::try_from(KABOOM_MOUNTS_TEMPLATE.chat_link).unwrap();
    let ChatLink::TravelTemplate(armor_template) = chat_link else {
        panic!("Expected TravelTemplate, got {chat_link:?}");
    };

    let merged = base_template.merge(&armor_template, false, true);

    for (slot, appearance) in &merged {
        if slot.kind() == FashionSlotKind::Mount {
            assert_eq!(appearance.skin(), armor_template.get_slot(&slot).skin());
            assert_eq!(
                appearance.is_visible(),
                armor_template.get_slot(&slot).is_visible()
            );
            assert_eq!(appearance.dyes(), base_template.get_slot(&slot).dyes());
        } else {
            assert_eq!(appearance, base_template.get_slot(&slot));
        }
    }
}

#[test]
#[test_log::test]
fn test_merge_peekaboo_with_kaboom_mounts_dyes_only() {
    let chat_link = &ChatLink::try_from(PEEKABOO_TEMPLATE.chat_link).unwrap();
    let ChatLink::TravelTemplate(base_template) = chat_link else {
        panic!("Expected TravelTemplate, got {chat_link:?}");
    };

    let chat_link = &ChatLink::try_from(KABOOM_MOUNTS_TEMPLATE.chat_link).unwrap();
    let ChatLink::TravelTemplate(armor_template) = chat_link else {
        panic!("Expected TravelTemplate, got {chat_link:?}");
    };

    let merged = base_template.merge(&armor_template, true, false);

    for (slot, appearance) in &merged {
        if slot.kind() == FashionSlotKind::Mount {
            assert_eq!(appearance.skin(), base_template.get_slot(&slot).skin());
            assert_eq!(
                appearance.is_visible(),
                base_template.get_slot(&slot).is_visible()
            );
            assert_eq!(appearance.dyes(), armor_template.get_slot(&slot).dyes());
        } else {
            assert_eq!(appearance, base_template.get_slot(&slot));
        }
    }
}
