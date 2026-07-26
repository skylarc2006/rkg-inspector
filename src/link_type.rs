#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinkType {
    Json,
    Html,
    Rkg,
    Mii,
}

impl LinkType {
    pub fn extension(self) -> &'static str {
        match self {
            LinkType::Json => "json",
            LinkType::Html => "html",
            LinkType::Rkg => "rkg",
            LinkType::Mii => "mii",
        }
    }
}
