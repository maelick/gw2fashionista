use gw2fashionista_chatlink::domain::{
    chatlink::ChatLink,
    templates::{
        SlotFilter, SlotFilterExt,
        wardrobe::{EquipmentCategory, WardrobeSlot, WardrobeTemplate},
    },
};
use gw2fashionista_fixtures::wardrobe::{ZIZI_ARMOR_TEMPLATE, ZIZI_TEMPLATE};

#[test]
#[test_log::test]
fn test_filter_zizi() {
    let template = &ZIZI_TEMPLATE.chat_link.parse::<WardrobeTemplate>().unwrap();

    let mut filter = SlotFilter::<WardrobeSlot>::all();
    filter.remove(&WardrobeSlot::Outfit);
    filter.remove_all(EquipmentCategory::Underwater.slots());
    filter.remove_all(EquipmentCategory::Weapons.slots());

    let filtered = template.filter(&filter);

    let filtered_link = &ChatLink::WardrobeTemplate(filtered);
    assert_eq!(
        filtered_link.to_string(),
        format!("[&{}]", ZIZI_ARMOR_TEMPLATE.chat_link)
    );
}
