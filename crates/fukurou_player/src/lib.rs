use fukurou_core::item_id::ItemId;

///
#[derive(Clone)]
pub struct Player {}

impl Player {
    pub fn load(&mut self, item_id: &ItemId, start_playing: bool, position_ms: u32) {
        // #TODO: Stop sink
        // #TODO: PlayerEvent::Loading
        // #TODO: Load file asynchronously
    }
}
