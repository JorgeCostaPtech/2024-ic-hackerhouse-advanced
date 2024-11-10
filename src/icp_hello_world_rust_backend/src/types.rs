use candid::{CandidType, Deserialize, Principal};
use candid::{Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

// Define a ChatMessage struct
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub sender: Principal,
    pub content: String,
    pub timestamp: u64,
} 

impl Storable for ChatMessage {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}