use super::{
    EventId,
    ShortGroupInfo
};
use authentication::UserInfo;
use types::{
    Id
};

#[derive(Clone, RustcEncodable)]
pub enum FeedEventState {
    Start,
    Finish
}

#[derive(RustcEncodable)]
pub struct FeedEventInfo {
    pub creation_time: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub event_id: EventId,
    pub scheduled_id: Id,
    pub event_name: String,
    pub state: FeedEventState,
    pub data: String,
    pub creator: Option<UserInfo>,
    pub group: ShortGroupInfo
}
