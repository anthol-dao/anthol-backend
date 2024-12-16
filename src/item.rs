use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use shared::{
    item::{attr::Stock, ItemId, ItemName},
    unit::{Currency, Price},
};
use std::{borrow::Cow, collections::BTreeMap};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ItemFluctuateData {
    V1 {
        id: ItemId,
        name: ItemName,
        stock: Stock,
        price: BTreeMap<Currency, Price>,
    },
}

impl Storable for ItemFluctuateData {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
