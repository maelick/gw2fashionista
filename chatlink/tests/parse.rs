use gw2fashionista_chatlink::{ChatLink, ChatLinkError};
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
