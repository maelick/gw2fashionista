use gw2fashionista_appearance::gw2::resolve::Resolver;
use gw2fashionista_appearance::models::skin::Skin;
use gw2fashionista_appearance::models::template::WardrobeTemplateData;
use gw2fashionista_chatlink::templates::wardrobe::{WardrobeSlot, WardrobeTemplate};
use std::assert_matches;

use gw2fashionista_fixtures::wardrobe::{EMPTY_TEMPLATE, ZIZI_ARMOR_TEMPLATE, ZIZI_TEMPLATE};

#[tokio::test(flavor = "multi_thread")]
#[test_log::test]
async fn test_resolve_empty() {
    let resolver = Resolver::default();
    let template = &EMPTY_TEMPLATE
        .chat_link
        .parse::<WardrobeTemplate>()
        .unwrap();

    resolver.cache_template(template).await.unwrap();

    let data = resolver.resolve_template(&template.into()).await.unwrap();
    assert!(data.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
#[test_log::test]
async fn test_resolve_zizi_armor() {
    let resolver = Resolver::default();
    let template = &ZIZI_ARMOR_TEMPLATE.chat_link.parse().unwrap();

    resolver.cache_template(template).await.unwrap();

    let data = resolver.resolve_template(&template.into()).await.unwrap();

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

#[tokio::test(flavor = "multi_thread")]
#[test_log::test]
async fn test_resolve_zizi() {
    let resolver = Resolver::default();
    let template = &ZIZI_TEMPLATE.chat_link.parse().unwrap();

    resolver.cache_template(template).await.unwrap();

    let data = &resolver.resolve_template(&template.into()).await.unwrap();

    assert_eq!(
        data.get(&WardrobeSlot::Aquabreather)
            .unwrap()
            .name
            .as_ref()
            .unwrap(),
        "Black Earth Aquabreather"
    );
    assert_eq!(
        data.get(&WardrobeSlot::Outfit)
            .unwrap()
            .name
            .as_ref()
            .unwrap(),
        "Hologram Outfit"
    );
    assert_eq!(
        data.get(&WardrobeSlot::WeaponAquaticA)
            .unwrap()
            .name
            .as_ref()
            .unwrap(),
        "Steam Speargun"
    );
    assert_eq!(
        data.get(&WardrobeSlot::WeaponAquaticB)
            .unwrap()
            .name
            .as_ref()
            .unwrap(),
        "Iron Spear"
    );
    assert_eq!(
        data.get(&WardrobeSlot::WeaponA1)
            .unwrap()
            .name
            .as_ref()
            .unwrap(),
        "Quip"
    );
    assert_eq!(
        data.get(&WardrobeSlot::WeaponA2)
            .unwrap()
            .name
            .as_ref()
            .unwrap(),
        "Quip"
    );
    assert_eq!(
        data.get(&WardrobeSlot::WeaponB1)
            .unwrap()
            .name
            .as_ref()
            .unwrap(),
        "The Dreamer"
    );
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
    assert_eq!(skin.name.as_ref().unwrap(), skin_name);
    let (d1, d2, d3, d4) = skin.dyes.as_ref().unwrap();
    assert_eq!(d1.name.clone().unwrap(), dye1_name);
    assert_eq!(d2.name.clone().unwrap(), dye2_name);
    assert_eq!(d3.name.clone().unwrap(), dye3_name);
    assert_eq!(d4.name.clone().unwrap(), dye4_name);
}
