pub mod travel;
pub mod wardrobe;

/// Represents a travel or wardrobe template test case
pub struct FashionTemplate {
    pub name: &'static str,
    pub chat_link: &'static str,
}

impl FashionTemplate {
    const fn new(name: &'static str, chat_link: &'static str) -> Self {
        FashionTemplate { name, chat_link }
    }

    pub fn snapshot_name(&self, prefix: &str) -> String {
        if prefix.is_empty() {
            self.name.to_string()
        } else {
            format!("{}_{}", prefix, self.name)
        }
    }
}

pub fn templates_as_csv(templates: &[FashionTemplate]) -> Vec<String> {
    templates
        .iter()
        .map(|t| format!("{},{}", t.name, t.chat_link))
        .collect()
}

pub fn templates_as_list(templates: &[FashionTemplate]) -> Vec<String> {
    templates.iter().map(|t| t.chat_link.to_string()).collect()
}
