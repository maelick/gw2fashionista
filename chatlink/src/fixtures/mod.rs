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

    pub fn snapshot_name(&self, prefix: &str) -> String {
        if prefix.is_empty() {
            self.name.to_string()
        } else {
            format!("{}_{}", prefix, self.name)
        }
    }

    pub fn wrap(&self) -> String {
        format!("[&{}]", self.link)
    }
}

pub fn templates_as_csv(templates: &[ChatLinkFixture]) -> Vec<String> {
    templates
        .iter()
        .map(|t| format!("{},{}", t.name, t.link))
        .collect()
}

pub fn templates_as_list(templates: &[ChatLinkFixture]) -> Vec<String> {
    templates.iter().map(|t| t.link.to_string()).collect()
}
