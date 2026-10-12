use gw2fashionista_chatlink::fixtures::{ChatLinkFixture, coins, travel, wardrobe};
use gw2fashionista_chatlink::{ChatLink, ChatLinkError};
use rstest::rstest;
use std::assert_matches;

#[test]
#[test_log::test]
fn test_parse_empty_link() {
    let raw = "";

    let result = raw.parse::<ChatLink>();
    assert_matches!(result, Err(ChatLinkError::EmptyPayload));

    let raw_with_brackets = format!("[&{}]", raw);
    let result_with_brackets = raw_with_brackets.parse::<ChatLink>();
    assert_matches!(result_with_brackets, Err(ChatLinkError::EmptyPayload));
}

#[test]
#[test_log::test]
fn test_parse_not_chat_link() {
    let raw = "This is not a chat link";

    let result = raw.parse::<ChatLink>();
    assert_matches!(result, Err(ChatLinkError::InvalidBase64(_)));

    let raw_with_brackets = format!("[&{}]", raw);
    let result_with_brackets = raw_with_brackets.parse::<ChatLink>();
    assert_matches!(result_with_brackets, Err(ChatLinkError::InvalidBase64(_)));
}

#[test]
#[test_log::test]
fn test_parse_invalid_base64() {
    let raw = "hello";

    let result = raw.parse::<ChatLink>();
    assert_matches!(result, Err(ChatLinkError::InvalidBase64(_)));

    let raw_with_brackets = format!("[&{}]", raw);
    let result_with_brackets = raw_with_brackets.parse::<ChatLink>();
    assert_matches!(result_with_brackets, Err(ChatLinkError::InvalidBase64(_)));
}

#[test]
#[test_log::test]
fn test_parse_invalid_link_type() {
    let raw = "abcd";

    let result = raw.parse::<ChatLink>();
    assert_matches!(result, Err(ChatLinkError::UnknownType(_)));

    let raw_with_brackets = format!("[&{}]", raw);
    let result_with_brackets = raw_with_brackets.parse::<ChatLink>();
    assert_matches!(result_with_brackets, Err(ChatLinkError::UnknownType(_)));
}

#[test]
#[test_log::test]
fn test_parse_invalid_length() {
    let raw = "DwAAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAAAAAAAAAD/fw==";

    let result = raw.parse::<ChatLink>();
    assert_matches!(result, Err(ChatLinkError::TruncatedData(_)));

    let raw_with_brackets = format!("[&{}]", raw);
    let result_with_brackets = raw_with_brackets.parse::<ChatLink>();
    assert_matches!(result_with_brackets, Err(ChatLinkError::TruncatedData(_)));
}

#[rstest]
#[case(coins::ZERO)]
#[case(coins::ONE)]
#[case(coins::ONE_TWO_THREE)]
#[case(coins::MAX)]
#[case(wardrobe::EMPTY_TEMPLATE)]
#[case(wardrobe::PEEKABOO_TEMPLATE)]
#[case(wardrobe::ZIZI_TEMPLATE)]
#[case(wardrobe::ZIZI_ARMOR_TEMPLATE)]
#[case(travel::EMPTY_TEMPLATE)]
#[case(travel::DEFAULT_MOUNT_TEMPLATE)]
#[case(travel::NO_DYES_TEMPLATE)]
#[case(travel::PEEKABOO_TEMPLATE)]
#[case(travel::KABOOM_TEMPLATE)]
#[case(travel::KABOOM_MOUNTS_TEMPLATE)]
#[case(travel::ZIZI_TEMPLATE)]
#[test_log::test]
fn test_parse_all_roundtrip(#[case] fixture: ChatLinkFixture) {
    let result: ChatLink = fixture.link.parse().unwrap();
    assert_eq!(result.link_type(), fixture.link_type);

    match result {
        ChatLink::Unparsed {
            link_type,
            bytes: _,
        } => panic!("{:?} chat link for {:?} is unparsed", link_type, fixture),
        _ => (),
    }

    insta::with_settings!({ prepend_module_to_snapshot => false }, {
        insta::assert_yaml_snapshot!(fixture.name, &result);
    });

    assert_eq!(result.to_string(), fixture.wrap());
    assert_eq!(result, fixture.wrap().parse().unwrap());
}
