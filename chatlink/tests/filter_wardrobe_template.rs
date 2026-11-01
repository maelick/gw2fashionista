use gw2fashionista_chatlink::fixtures::wardrobe::{ZIZI, ZIZI_ARMOR};
use gw2fashionista_chatlink::{
    ChatLink,
    templates::{
        SlotFilter, SlotFilterExt,
        wardrobe::{EquipmentCategory, WardrobeSlot, WardrobeTemplate},
    },
};

#[test]
#[test_log::test]
fn test_filter_zizi() {
    let template = &ZIZI.link.parse::<WardrobeTemplate>().unwrap();

    let mut filter = SlotFilter::<WardrobeSlot>::all();
    filter.remove(&WardrobeSlot::Outfit);
    filter.remove_all(EquipmentCategory::Underwater.slots());
    filter.remove_all(EquipmentCategory::Weapons.slots());

    let filtered = template.filter(&filter);

    let filtered_link = &ChatLink::WardrobeTemplate(filtered);
    assert_eq!(filtered_link.to_string(), ZIZI_ARMOR.wrap(),);
}
