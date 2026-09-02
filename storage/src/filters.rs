use bon::Builder;

#[derive(Debug, Clone, Eq, PartialEq, Default, Builder)]
pub struct StringFilters {
    #[builder(into)]
    prefix: Option<String>,

    #[builder(into)]
    suffix: Option<String>,

    #[builder(default)]
    #[builder(with = |s: impl IntoIterator<Item: Into<String>>| collect_strings(s))]
    substrings: Vec<String>,
}

impl StringFilters {
    pub fn patterns(&self) -> impl Iterator<Item = String> {
        self.substrings
            .iter()
            .map(|s| format!("%{}%", s))
            .chain(self.prefix.as_ref().map(|s| format!("{}%", s)))
            .chain(self.suffix.as_ref().map(|s| format!("%{}", s)))
    }
}

fn collect_strings(strings: impl IntoIterator<Item: Into<String>>) -> Vec<String> {
    strings.into_iter().map(Into::into).collect()
}
