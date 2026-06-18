#[cfg(test)]
mod tests {
    use std::assert_matches;
    use gw2fashionista_core::domain::chatlink::ChatLink;
    use gw2fashionista_core::domain::wardrobe_template::WardrobeTemplate;
    use gw2fashionista_core::gw2_data::Resolver;
    use gw2fashionista_core::models::skin::Skin;
    use gw2fashionista_core::models::wardrobe_template::WardrobeTemplateData;

    const EMPTY_TEMPLATE: &str = "DwAAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAAAAAAAAAAAAAD/fw==";
    const ZIZI_ARMOR_TEMPLATE: &str = "DwAAPQkBAAEAAQABAAwAGAURBhEGAQAjABgFEQYBAAEA/AABABEGGAUYBdIDGAURBgEAAQALAAEAEQYRBgEAohYYBREGAQABAAAAAQABAAEAAQAAAAAAAAAAAAAAAAD/fw==";
    const ZIZI_TEMPLATE: &str = "D1sDPQkBAAEAAQABAAwAGAURBhEGAQAjABgFEQYBAAEA/AABABEGGAUYBdIDGAURBgEAAQALAAEAEQYRBgEAohYYBREGAQABAHwAAQABAAEAAQDjE6APPBI8Ej0SAAD+fg==";

    #[test]
    fn test_resolve_empty() {
        let mut resolver = Resolver::default();
        let template = parse_template(EMPTY_TEMPLATE);

        resolver.cache_wardrobe_template(&template).unwrap();

        let data = resolver.resolve_wardrobe_template(&template);

        assert_matches!(data.aquabreather, None);
        assert_matches!(data.backpack, None);
        assert_matches!(data.chest, None);
        assert_matches!(data.shoes, None);
        assert_matches!(data.gloves, None);
        assert_matches!(data.head, None);
        assert_matches!(data.legs, None);
        assert_matches!(data.shoulders, None);
        assert_matches!(data.outfit, None);
        assert_matches!(data.weapon_aquatic_a, None);
        assert_matches!(data.weapon_aquatic_b, None);
        assert_matches!(data.weapon_a1, None);
        assert_matches!(data.weapon_a2, None);
        assert_matches!(data.weapon_b1, None);
        assert_matches!(data.weapon_b2, None);
    }

    #[test]
    fn test_resolve_zizi_armor() {
        let mut resolver = Resolver::default();
        let template = parse_template(ZIZI_ARMOR_TEMPLATE);

        resolver.cache_wardrobe_template(&template).unwrap();

        let data = resolver.resolve_wardrobe_template(&template);

        assert_matches!(data.aquabreather, None);
        assert_matches!(data.outfit, None);
        assert_matches!(data.weapon_aquatic_a, None);
        assert_matches!(data.weapon_aquatic_b, None);
        assert_matches!(data.weapon_a1, None);
        assert_matches!(data.weapon_a2, None);
        assert_matches!(data.weapon_b1, None);
        assert_matches!(data.weapon_b2, None);

        assert_zizi_armor(&data);
    }

    #[test]
    fn test_resolve_zizi() {
        let mut resolver = Resolver::default();
        let template = parse_template(ZIZI_TEMPLATE);

        resolver.cache_wardrobe_template(&template).unwrap();

        let data = &resolver.resolve_wardrobe_template(&template);

        assert_matches!(&data.aquabreather.as_ref().unwrap().name, Some(name) if name == "Black Earth Aquabreather");
        assert_matches!(&data.outfit.as_ref().unwrap().name, Some(name) if name == "Hologram Outfit");
        assert_matches!(&data.weapon_aquatic_a.as_ref().unwrap().name, Some(name) if name == "Steam Speargun");
        assert_matches!(&data.weapon_aquatic_b.as_ref().unwrap().name, Some(name) if name == "Iron Spear");
        assert_matches!(&data.weapon_a1.as_ref().unwrap().name, Some(name) if name == "Quip");
        assert_matches!(&data.weapon_a2.as_ref().unwrap().name, Some(name) if name == "Quip");
        assert_matches!(&data.weapon_b1.as_ref().unwrap().name, Some(name) if name == "The Dreamer");
        assert_matches!(&data.weapon_b2, None);

        assert_zizi_armor(&data);
    }

    fn assert_zizi_armor(data: &WardrobeTemplateData) {
        assert_dyable_skin(data.backpack.as_ref().unwrap(), "Pink Quaggan Backpack", "Dye Remover", "Dye Remover", "Dye Remover", "Dye Remover");
        assert_dyable_skin(data.chest.as_ref().unwrap(), "Sneakthief Coat", "Electro Pink", "Permafrost", "Permafrost", "Dye Remover");
        assert_dyable_skin(data.shoes.as_ref().unwrap(), "Sneakthief Sandals", "Electro Pink", "Permafrost", "Dye Remover", "Dye Remover");
        assert_dyable_skin(data.gloves.as_ref().unwrap(), "Noble Gloves", "Dye Remover", "Permafrost", "Electro Pink", "Electro Pink");
        assert_dyable_skin(data.head.as_ref().unwrap(), "Fuzzy Cat Hat", "Electro Pink", "Permafrost", "Dye Remover", "Dye Remover");
        assert_dyable_skin(data.legs.as_ref().unwrap(), "Sneakthief Leggings", "Dye Remover", "Permafrost", "Permafrost", "Dye Remover");
        assert_dyable_skin(data.shoulders.as_ref().unwrap(), "Shoulder Scarf", "Electro Pink", "Permafrost", "Dye Remover", "Dye Remover");
    }

    fn assert_dyable_skin(skin: &Skin, skin_name: &str, dye1_name: &str, dye2_name: &str, dye3_name: &str, dye4_name: &str) {
        assert_matches!(&skin.name, Some(name) if name == skin_name);
        assert_matches!(&skin.dyes.as_ref().unwrap().0.name, Some(name) if name == dye1_name);
        assert_matches!(&skin.dyes.as_ref().unwrap().1.name, Some(name) if name == dye2_name);
        assert_matches!(&skin.dyes.as_ref().unwrap().2.name, Some(name) if name == dye3_name);
        assert_matches!(&skin.dyes.as_ref().unwrap().3.name, Some(name) if name == dye4_name);
    }

    fn parse_template(raw_chat_link: &str) -> WardrobeTemplate {
        let chat_link = ChatLink::try_from(raw_chat_link).unwrap();
        let ChatLink::WardrobeTemplate(template) = chat_link else {
           panic!("Expected WardrobeTemplate, got {chat_link:?}");
        };
        template
    }
}
