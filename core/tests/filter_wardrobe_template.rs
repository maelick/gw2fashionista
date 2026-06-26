#[cfg(test)]
mod tests {
    use gw2fashionista_core::domain::{chatlink::ChatLink, wardrobe_template::slot::{EquipmentCategory, SlotFilter, SlotFilterExt, SlotType}};
    use gw2fashionista_fixtures::wardrobe::{ZIZI_TEMPLATE, ZIZI_ARMOR_TEMPLATE};

    #[test]
    fn test_filter_zizi() {
        let chat_link = &ChatLink::try_from(ZIZI_TEMPLATE).unwrap();

        let ChatLink::WardrobeTemplate(template) = chat_link else {
           panic!("Expected WardrobeTemplate, got {chat_link:?}");
        };

        let mut filter = SlotFilter::all();
        filter.remove(&SlotType::Outfit);
        filter.remove_all(EquipmentCategory::Underwater.slots());
        filter.remove_all(EquipmentCategory::Weapons.slots());

        let filtered = template.filter(&filter);

        let filtered_link = &ChatLink::WardrobeTemplate(filtered);
        let filtered_link: String = filtered_link.try_into().unwrap();
        assert_eq!(filtered_link, format!("[&{}]", ZIZI_ARMOR_TEMPLATE));
    }
}
