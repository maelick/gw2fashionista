use bon::Builder;

#[derive(Debug, Clone, Eq, PartialEq, Default, Builder)]
pub struct StringFilters {
    #[builder(into)]
    pub prefix: Option<String>,

    #[builder(into)]
    pub suffix: Option<String>,

    #[builder(default)]
    #[builder(with = |s: impl IntoIterator<Item: Into<String>>| collect_strings(s))]
    pub substrings: Vec<String>,
}

fn collect_strings(strings: impl IntoIterator<Item: Into<String>>) -> Vec<String> {
    strings.into_iter().map(Into::into).collect()
}
