use crate::ChatLinkType;

pub mod travel;
pub mod wardrobe;

#[derive(Clone, Copy, Debug)]
/// Represents a chat link test fixture
pub struct ChatLinkFixture {
    pub name: &'static str,
    pub link_type: ChatLinkType,
    pub link: &'static str,
}

impl ChatLinkFixture {
    const fn new(name: &'static str, link_type: ChatLinkType, link: &'static str) -> Self {
        ChatLinkFixture {
            name,
            link_type,
            link,
        }
    }

    pub fn wrap(&self) -> String {
        format!("[&{}]", self.link)
    }
}
