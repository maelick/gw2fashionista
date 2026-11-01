use gw2fashionista_chatlink::fixtures::travel::{KABOOM, KABOOM_MOUNTS};
use gw2fashionista_chatlink::{
    ChatLink,
    templates::{
        SlotFilter, SlotFilterExt,
        travel::{TravelCategory, TravelSlot, TravelTemplate},
    },
};

#[test]
#[test_log::test]
fn test_filter_kaboom() {
    let template = &KABOOM.link.parse::<TravelTemplate>().unwrap();

    let mut filter = SlotFilter::<TravelSlot>::all();
    filter.retain_all(TravelCategory::Mounts.slots());

    let filtered = template.filter(&filter);

    let filtered_link = &ChatLink::TravelTemplate(filtered);
    assert_eq!(filtered_link.to_string(), KABOOM_MOUNTS.wrap(),);
}
