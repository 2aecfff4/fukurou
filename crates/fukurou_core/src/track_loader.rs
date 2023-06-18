use crate::{file_id::FileId, item_id::ItemId};
use thiserror::Error;

///
#[derive(Error, Debug)]
pub enum TrackLoaderError {}

///
#[derive(Clone)]
pub struct LoadResult {
    file_id: FileId,
    // #TODO: File type, bitrate, sample rate etc
}

///
#[async_trait::async_trait]
pub trait TrackLoader {
    ///
    async fn load(&mut self, item_id: &ItemId) -> Result<LoadResult, TrackLoaderError>;
}
