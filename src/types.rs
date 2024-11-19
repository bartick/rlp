#[derive(Debug, Clone)]
pub enum RLPItem {
    Str(String),
    List(Vec<RLPItem>),
}

impl From<&str> for RLPItem {
    fn from(s: &str) -> Self {
        RLPItem::Str(s.to_string())
    }
}

impl From<Vec<RLPItem>> for RLPItem {
    fn from(v: Vec<RLPItem>) -> Self {
        RLPItem::List(v)
    }
}