use gw2fashionista_core::domain::{
    chatlink::ChatLink,
    templates::{
        SlotFilter, SlotFilterExt,
        travel::{TravelCategory, TravelSlot},
    },
};
use gw2fashionista_fixtures::travel::{KABOOM_MOUNTS_TEMPLATE, KABOOM_TEMPLATE};

#[test]
#[test_log::test]
fn test_filter_kaboom() {
    let chat_link = &ChatLink::try_from(KABOOM_TEMPLATE.chat_link).unwrap();

    let ChatLink::TravelTemplate(template) = chat_link else {
        panic!("Expected TravelTemplate, got {chat_link:?}");
    };

    let mut filter = SlotFilter::<TravelSlot>::all();
    filter.retain_all(TravelCategory::Mounts.slots());

    let filtered = template.filter(&filter);

    let filtered_link = &ChatLink::TravelTemplate(filtered);
    let filtered_link: String = filtered_link.try_into().unwrap();
    assert_eq!(
        filtered_link,
        format!("[&{}]", KABOOM_MOUNTS_TEMPLATE.chat_link)
    );
}
