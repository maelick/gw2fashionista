use gw2fashionista_core::domain::templates::travel::TravelTemplate;
use gw2fashionista_core::domain::{chatlink::ChatLink, templates::travel::TravelSlot};
use gw2fashionista_core::gw2_data::Resolver;
use gw2fashionista_core::models::skin::Skin;

use gw2fashionista_fixtures::travel::{EMPTY_TEMPLATE, PEEKABOO_TEMPLATE, ZIZI_TEMPLATE};

#[tokio::test]
async fn test_resolve_empty() {
    let resolver = Resolver::default();
    let template = &parse_template(EMPTY_TEMPLATE.chat_link);

    // resolver.cache_travel_template(&template).await.unwrap();

    let data = resolver
        .resolve_template(&template.into())
        .await
        .unwrap();
    assert!(data.is_empty());
}

#[tokio::test]
async fn test_resolve_peekaboo() {
    let resolver = Resolver::default();
    let template = &parse_template(PEEKABOO_TEMPLATE.chat_link);

    // resolver.cache_travel_template(&template).await.unwrap();

    let data = resolver
        .resolve_template(&template.into())
        .await
        .unwrap();

    for (slot, skin) in data.into_iter() {
        match slot {
            TravelSlot::Glider => assert_dyeable_skin(
                skin,
                "Orrax Manifested",
                "Celestial",
                "Violite",
                "Violite",
                "Glint's Rebellion",
            ),
            TravelSlot::Doorway => assert_dyeable_skin(
                skin,
                "Unknown", // It seems there is no API endpoint to get doorway data
                "Glint's Rebellion",
                "Violite",
                "Violite",
                "Violite",
            ),
            TravelSlot::Jackal => assert_dyeable_skin(
                skin,
                "Plush Vulpine Jackal",
                "Glint's Rebellion",
                "Violite",
                "Glint's Rebellion",
                "Violite",
            ),
            TravelSlot::Griffon => assert_dyeable_skin(
                skin,
                "Plush Griffon",
                "Celestial",
                "Glint's Rebellion",
                "Shadow Purple",
                "Violite",
            ),
            TravelSlot::Springer => assert_dyeable_skin(
                skin,
                "Plush Cuckoo",
                "Violite",
                "Glint's Rebellion",
                "Shadow Purple",
                "Glint's Rebellion",
            ),
            TravelSlot::Skimmer => assert_dyeable_skin(
                skin,
                "Unknown", // TODO: investigate why this is missing
                "Violite",
                "Violite",
                "Glint's Rebellion",
                "Shadow Purple",
            ),
            TravelSlot::Raptor => assert_dyeable_skin(
                skin,
                "Plush Raptor",
                "Celestial",
                "Glint's Rebellion",
                "Violite",
                "Shadow Purple",
            ),
            TravelSlot::Beetle => assert_dyeable_skin(
                skin,
                "Panda Roller Beetle",
                "Shadow Purple",
                "Glint's Rebellion",
                "Violite",
                "Violite",
            ),
            TravelSlot::Warclaw => assert_dyeable_skin(
                skin,
                "Plush Tiger Warclaw",
                "Celestial",
                "Glint's Rebellion",
                "Violite",
                "Shadow Purple",
            ),
            TravelSlot::Skyscale => assert_dyeable_skin(
                skin,
                "Plush Skyscale",
                "Violite",
                "Celestial",
                "Celestial",
                "Scenic",
            ),
            TravelSlot::Skiff => assert_dyeable_skin(
                skin,
                "Floating Garden",
                "Glint's Rebellion",
                "Violite",
                "Glint's Rebellion",
                "Violite",
            ),
            TravelSlot::Turtle => assert_dyeable_skin(
                skin,
                "Plush Siege Turtle",
                "Glint's Rebellion",
                "Violite",
                "Shadow Purple",
                "Glint's Rebellion",
            ),
        }
    }
}

#[tokio::test]
async fn test_resolve_zizi() {
    let resolver = Resolver::default();
    let template = &parse_template(ZIZI_TEMPLATE.chat_link);

    // resolver.cache_travel_template(&template).await.unwrap();

    let data = &resolver
        .resolve_template(&template.into())
        .await
        .unwrap();

    for (slot, skin) in data.into_iter() {
        match slot {
            TravelSlot::Glider => assert_dyeable_skin(
                skin,
                "Dynamics Hoverboard Glider",
                "Electro Pink",
                "Electro Pink",
                "Permafrost",
                "Permafrost",
            ),
            TravelSlot::Doorway => assert_dyeable_skin(
                skin,
                "Unknown", // It seems there is no API endpoint to get doorway data
                "Hot Pink",
                "Jalapeño",
                "Electro Pink",
                "Mullberry",
            ),
            TravelSlot::Jackal => assert_dyeable_skin(
                skin,
                "Plush Vulpine Jackal",
                "White",
                "Electro Pink",
                "Electro Pink",
                "Electro Pink",
            ),
            TravelSlot::Griffon => assert_dyeable_skin(
                skin,
                "Plush Griffon",
                "Permafrost",
                "Electro Pink",
                "Electro Pink",
                "Electro Pink",
            ),
            TravelSlot::Springer => assert_dyeable_skin(
                skin,
                "Plush Cuckoo",
                "Electro Pink",
                "Permafrost",
                "Electro Pink",
                "Electro Pink",
            ),
            TravelSlot::Skimmer => assert_dyeable_skin(
                skin,
                "Unknown", // TODO: investigate why this is missing
                "Permafrost",
                "Electro Pink",
                "Hot Pink",
                "Hot Pink",
            ),
            TravelSlot::Raptor => assert_dyeable_skin(
                skin,
                "Plush Raptor",
                "Permafrost",
                "Electro Pink",
                "Royal Rose",
                "Eternal Ice",
            ),
            TravelSlot::Beetle => assert_dyeable_skin(
                skin,
                "Panda Roller Beetle",
                "Electro Pink",
                "Permafrost",
                "Electro Pink",
                "Abyss",
            ),
            TravelSlot::Warclaw => assert_dyeable_skin(
                skin,
                "Plush Tiger Warclaw",
                "Permafrost",
                "Electro Pink",
                "Royal Rose",
                "Hot Pink",
            ),
            TravelSlot::Skyscale => assert_dyeable_skin(
                skin,
                "Plush Skyscale",
                "Electro Pink",
                "Permafrost",
                "Permafrost",
                "Electro Pink",
            ),
            TravelSlot::Skiff => assert_dyeable_skin(
                skin,
                "Shing Jea Dragon Boat",
                "Permafrost",
                "Electro Pink",
                "Electro Pink",
                "Permafrost",
            ),
            TravelSlot::Turtle => assert_dyeable_skin(
                skin,
                "Plush Siege Turtle",
                "Permafrost",
                "Electro Pink",
                "Electro Pink",
                "Permafrost",
            ),
        }
    }
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

fn parse_template(raw_chat_link: &str) -> TravelTemplate {
    let chat_link = ChatLink::try_from(raw_chat_link).unwrap();
    let ChatLink::TravelTemplate(template) = chat_link else {
        panic!("Expected TravelTemplate, got {chat_link:?}");
    };
    template
}
