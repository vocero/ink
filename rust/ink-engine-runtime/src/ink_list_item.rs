pub struct InkListItem {
    full_name: String,
    origin_name: String,
    item_name: String,
}

impl InkListItem {
    pub fn new(origin_name: &str, item_name: &str) -> Self {
        Self {
            full_name: format!("{}.{}", origin_name, item_name),
            origin_name: origin_name.to_string(),
            item_name: item_name.to_string(),
        }
    }

    pub fn from_full_name(full_name: &str) -> Option<Self> {
        let mut parts = full_name.split('.');

        Some(Self {
            full_name: full_name.to_string(),
            origin_name: parts.next()?.to_string(),
            item_name: parts.next()?.to_string(),
        })
    }

    pub fn origin_name(&self) -> &str { self.origin_name.as_ref() }

    pub fn item_name(&self) -> &str { self.item_name.as_ref() }

    pub fn full_name(&self) -> &str { &self.full_name.as_ref() }
}
