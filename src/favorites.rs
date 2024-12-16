use shared::{item::ItemId, market::MarketId, store::StoreId};

pub struct FavoriteItemV1 {
    pub item_id: ItemId,
    pub store_id: StoreId,
    pub market_id: MarketId,
    pub added_at: u64,
}
