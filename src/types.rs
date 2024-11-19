#[derive(Debug, Clone, PartialEq, Eq)]
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

impl std::fmt::Display for RLPItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RLPItem::Str(s) => write!(f, "{}", s),
            RLPItem::List(v) => {
                write!(f, "[")?;
                for item in v {
                    write!(f, "{}, ", item)?;
                }
                write!(f, "]")
            }
        }
    }
}

// impl AddAssign for RLPItem
impl std::ops::AddAssign for RLPItem {
    fn add_assign(&mut self, other: Self) {
        match self {
            RLPItem::List(ref mut v) => {
                v.push(other);
            }
            RLPItem::Str(ref mut v) => {
                let mut new_v = v.clone();
                new_v.push_str(&other.to_string());
                *self = RLPItem::Str(new_v);
            }
        }
    }
}