use gw2fashionista_chatlink::domain::{
    chatlink::ChatLink,
    templates::{
        SlotFilter, SlotFilterExt,
        travel::{TravelCategory, TravelSlot, TravelTemplate},
    },
};
use gw2fashionista_fixtures::travel::{KABOOM_MOUNTS_TEMPLATE, KABOOM_TEMPLATE};

#[test]
#[test_log::test]
fn test_filter_kaboom() {
    let template = &KABOOM_TEMPLATE.chat_link.parse::<TravelTemplate>().unwrap();

    let mut filter = SlotFilter::<TravelSlot>::all();
    filter.retain_all(TravelCategory::Mounts.slots());

    let filtered = template.filter(&filter);

    let filtered_link = &ChatLink::TravelTemplate(filtered);
    assert_eq!(
        filtered_link.to_string(),
        format!("[&{}]", KABOOM_MOUNTS_TEMPLATE.chat_link)
    );
}
