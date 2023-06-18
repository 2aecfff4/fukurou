///
#[derive(Clone, PartialEq, Eq)]
pub struct FileId(Vec<u8>);

impl FileId {
    pub fn id(&self) -> &[u8] {
        &self.0
    }
}
