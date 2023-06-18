///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemType {
    Album,
    Artist,
    Playlist,
    Track,
    Unknown,
}

///
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ItemId {
    provider: String,
    item_type: ItemType,
    id: String,
}

impl ItemId {
    pub fn new(provider: String, item_type: ItemType, id: String) -> Self {
        Self {
            provider,
            item_type,
            id,
        }
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }

    pub fn item_type(&self) -> ItemType {
        self.item_type
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
