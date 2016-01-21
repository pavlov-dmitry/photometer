use super::{
    EventId
};
use types::{
    Id,
    ShortInfo
};

#[derive(Clone, RustcEncodable)]
pub enum FeedEventState {
    Start,
    Finish
}

#[derive(RustcEncodable)]
pub struct FeedEventInfo {
    pub id: Id,
    pub creation_time: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub event_id: EventId,
    pub scheduled_id: Id,
    pub event_name: String,
    pub state: FeedEventState,
    pub data: String,
    pub creator: Option<ShortInfo>,
    pub group: ShortInfo
}
