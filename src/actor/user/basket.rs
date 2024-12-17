use candid::{CandidType, Deserialize};
use common::{
    item::{
        attr::{AttrIndexesResponse, AttrKeys},
        ItemKey, ItemName,
    },
    market::MarketPrincipal,
    media::MediaData,
    store::StorePrincipal,
};

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq)]
pub enum Basket {
    V1(BasketV1),
}

impl Default for Basket {
    fn default() -> Self {
        Basket::V1(BasketV1::default())
    }
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct BasketV1 {
    pub physical_items: Vec<PhysicalItemGroupOfSender>,
    pub digital_items: Vec<DigitalItemInBasket>,
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq)]
pub enum PhysicalItemGroupOfSender {
    V1(PhysicalItemGroupOfSenderV1),
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq)]
pub struct PhysicalItemGroupOfSenderV1 {
    pub store_principal: StorePrincipal,
    pub items: Vec<PhysicalItemInBasket>,
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq)]
pub enum PhysicalItemInBasket {
    V1(PhysicalItemInBasketV1),
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq)]
pub struct PhysicalItemInBasketV1 {
    pub market_principal: MarketPrincipal,
    pub item_key: ItemKey,
    pub image: MediaData,
    pub attr_keys: AttrKeys,
    pub attrs: AttrIndexesResponse,
    pub count: u32,
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq)]
pub enum DigitalItemInBasket {
    V1(DigitalItemInBasketV1),
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq)]
pub struct DigitalItemInBasketV1 {
    pub market_principal: MarketPrincipal,
    pub store_principal: StorePrincipal,
    pub item_key: ItemKey,
    pub item_name: ItemName,
    pub image: MediaData,
    pub attr_keys: AttrKeys,
    pub attrs: AttrIndexesResponse,
    pub count: u32,
}
