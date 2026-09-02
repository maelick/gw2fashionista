use gw2fashionista_chatlink::domain::templates::{
    FashionSlot, FashionSlotKind, travel::TravelTemplate,
};
use gw2fashionista_fixtures::travel::{KABOOM_MOUNTS_TEMPLATE, PEEKABOO_TEMPLATE};

#[test]
#[test_log::test]
fn test_merge_peekaboo_with_kaboom_mounts() {
    let base_template = &PEEKABOO_TEMPLATE
        .chat_link
        .parse::<TravelTemplate>()
        .unwrap();

    let mount_template = &KABOOM_MOUNTS_TEMPLATE
        .chat_link
        .parse::<TravelTemplate>()
        .unwrap();

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
    let base_template = &PEEKABOO_TEMPLATE
        .chat_link
        .parse::<TravelTemplate>()
        .unwrap();

    let mount_template = &KABOOM_MOUNTS_TEMPLATE
        .chat_link
        .parse::<TravelTemplate>()
        .unwrap();

    let merged = base_template.merge(&mount_template, false, true);

    for (slot, appearance) in &merged {
        if slot.kind() == FashionSlotKind::Mount {
            assert_eq!(appearance.skin(), mount_template.get_slot(&slot).skin());
            assert_eq!(
                appearance.is_visible(),
                mount_template.get_slot(&slot).is_visible()
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
    let base_template = &PEEKABOO_TEMPLATE
        .chat_link
        .parse::<TravelTemplate>()
        .unwrap();

    let mount_template = &KABOOM_MOUNTS_TEMPLATE
        .chat_link
        .parse::<TravelTemplate>()
        .unwrap();

    let merged = base_template.merge(&mount_template, true, false);

    for (slot, appearance) in &merged {
        if slot.kind() == FashionSlotKind::Mount {
            assert_eq!(appearance.skin(), base_template.get_slot(&slot).skin());
            assert_eq!(
                appearance.is_visible(),
                base_template.get_slot(&slot).is_visible()
            );
            assert_eq!(appearance.dyes(), mount_template.get_slot(&slot).dyes());
        } else {
            assert_eq!(appearance, base_template.get_slot(&slot));
        }
    }
}
