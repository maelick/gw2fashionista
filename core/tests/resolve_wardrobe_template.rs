use gw2fashionista_core::domain::chatlink::ChatLink;
use gw2fashionista_core::domain::templates::wardrobe::{WardrobeSlot, WardrobeTemplate};
use gw2fashionista_core::gw2_data::Resolver;
use gw2fashionista_core::models::skin::Skin;
use gw2fashionista_core::models::template::WardrobeTemplateData;
use std::assert_matches;

use gw2fashionista_fixtures::wardrobe::{EMPTY_TEMPLATE, ZIZI_ARMOR_TEMPLATE, ZIZI_TEMPLATE};

#[tokio::test]
async fn test_resolve_empty() {
    let resolver = Resolver::default();
    let template = parse_template(EMPTY_TEMPLATE.chat_link);

    resolver.cache_wardrobe_template(&template).await.unwrap();

    let data = resolver.resolve_wardrobe_template(&template).await.unwrap();
    assert!(data.is_empty());
}

#[tokio::test]
async fn test_resolve_zizi_armor() {
    let resolver = Resolver::default();
    let template = parse_template(ZIZI_ARMOR_TEMPLATE.chat_link);

    resolver.cache_wardrobe_template(&template).await.unwrap();

    let data = resolver.resolve_wardrobe_template(&template).await.unwrap();

    assert_matches!(data.get(&WardrobeSlot::Aquabreather), None);
    assert_matches!(data.get(&WardrobeSlot::Outfit), None);
    assert_matches!(data.get(&WardrobeSlot::WeaponAquaticA), None);
    assert_matches!(data.get(&WardrobeSlot::WeaponAquaticB), None);
    assert_matches!(data.get(&WardrobeSlot::WeaponA1), None);
    assert_matches!(data.get(&WardrobeSlot::WeaponA2), None);
    assert_matches!(data.get(&WardrobeSlot::WeaponB1), None);
    assert_matches!(data.get(&WardrobeSlot::WeaponB2), None);

    assert_zizi_armor(&data);
}

#[tokio::test]
async fn test_resolve_zizi() {
    let resolver = Resolver::default();
    let template = parse_template(ZIZI_TEMPLATE.chat_link);

    resolver.cache_wardrobe_template(&template).await.unwrap();

    let data = &resolver.resolve_wardrobe_template(&template).await.unwrap();

    assert_matches!(&data.get(&WardrobeSlot::Aquabreather).unwrap().name, Some(name) if name == "Black Earth Aquabreather");
    assert_matches!(&data.get(&WardrobeSlot::Outfit).unwrap().name, Some(name) if name == "Hologram Outfit");
    assert_matches!(&data.get(&WardrobeSlot::WeaponAquaticA).unwrap().name, Some(name) if name == "Steam Speargun");
    assert_matches!(&data.get(&WardrobeSlot::WeaponAquaticB).unwrap().name, Some(name) if name == "Iron Spear");
    assert_matches!(&data.get(&WardrobeSlot::WeaponA1).unwrap().name, Some(name) if name == "Quip");
    assert_matches!(&data.get(&WardrobeSlot::WeaponA2).unwrap().name, Some(name) if name == "Quip");
    assert_matches!(&data.get(&WardrobeSlot::WeaponB1).unwrap().name, Some(name) if name == "The Dreamer");
    assert_matches!(data.get(&WardrobeSlot::WeaponB2), None);

    assert_zizi_armor(&data);
}

fn assert_zizi_armor(data: &WardrobeTemplateData) {
    assert_dyeable_skin(
        data.get(&WardrobeSlot::Backpack).unwrap(),
        "Pink Quaggan Backpack",
        "Dye Remover",
        "Dye Remover",
        "Dye Remover",
        "Dye Remover",
    );
    assert_dyeable_skin(
        data.get(&WardrobeSlot::Chest).unwrap(),
        "Sneakthief Coat",
        "Electro Pink",
        "Permafrost",
        "Permafrost",
        "Dye Remover",
    );
    assert_dyeable_skin(
        data.get(&WardrobeSlot::Shoes).unwrap(),
        "Sneakthief Sandals",
        "Electro Pink",
        "Permafrost",
        "Dye Remover",
        "Dye Remover",
    );
    assert_dyeable_skin(
        data.get(&WardrobeSlot::Gloves).unwrap(),
        "Noble Gloves",
        "Dye Remover",
        "Permafrost",
        "Electro Pink",
        "Electro Pink",
    );
    assert_dyeable_skin(
        data.get(&WardrobeSlot::Head).unwrap(),
        "Fuzzy Cat Hat",
        "Electro Pink",
        "Permafrost",
        "Dye Remover",
        "Dye Remover",
    );
    assert_dyeable_skin(
        data.get(&WardrobeSlot::Legs).unwrap(),
        "Sneakthief Leggings",
        "Dye Remover",
        "Permafrost",
        "Permafrost",
        "Dye Remover",
    );
    assert_dyeable_skin(
        data.get(&WardrobeSlot::Shoulders).unwrap(),
        "Shoulder Scarf",
        "Electro Pink",
        "Permafrost",
        "Dye Remover",
        "Dye Remover",
    );
}

fn assert_dyeable_skin(
    skin: &Skin,
    skin_name: &str,
    dye1_name: &str,
    dye2_name: &str,
    dye3_name: &str,
    dye4_name: &str,
) {
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
