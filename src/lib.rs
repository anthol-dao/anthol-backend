#[macro_use]
extern crate nestify;

use anthol_store::item::AttrKeys;
use ic_cdk_macros::*;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    {DefaultMemoryImpl, StableBTreeMap, StableLog},
};
use common::{
    actor::{
        account::{
            AccountPageError, AccountPageResponse, SetUserProfileError, SetUserProfileRequest,
        },
        id::ActorId,
        ActorPrincipal,
    },
    item::{ItemKey, Tag},
    market::{MarketId, MarketPrincipal},
    route::home::{HomePageError, HomePageRequest, HomePageResponse},
    store::{StoreId, StorePrincipal},
    unit::Currency,
};
use std::cell::RefCell;

mod actor;
mod favorites;
mod item;
mod log;
mod market;
mod route;
mod store;
mod tag;
mod test;

#[cfg(feature = "canbench-rs")]
mod benches;

use actor::ActorData;
use item::ItemFluctuateData;
use market::MarketData;
use store::StoreData;
use tag::TagData;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub(crate) static LOG: RefCell<StableLog<(), Memory, Memory>> = RefCell::new(
        StableLog::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        ).unwrap()
    );

    pub(crate) static ACTORS: RefCell<StableBTreeMap<ActorPrincipal, ActorData, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
        )
    );

    pub(crate) static ACTORS_IN_ID: RefCell<StableBTreeMap<ActorId, ActorPrincipal, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
        )
    );

    pub(crate) static STORES: RefCell<StableBTreeMap<StorePrincipal, StoreData, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))),
        )
    );

    pub(crate) static STORES_IN_ID: RefCell<StableBTreeMap<StoreId, StorePrincipal, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))),
        )
    );

    pub(crate) static MARKETS: RefCell<StableBTreeMap<MarketPrincipal, MarketData, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6))),
        )
    );

    pub(crate) static MARKETS_IN_ID: RefCell<StableBTreeMap<MarketId, MarketPrincipal, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7))),
        )
    );

    pub(crate) static TAGS: RefCell<StableBTreeMap<Tag, TagData, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(8))),
        )
    );

    pub(crate) static ITEM_FLUCTUATE_DATA: RefCell<StableBTreeMap<(StorePrincipal, ItemKey, AttrKeys), ItemFluctuateData, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(9))),
        )
    );
}

#[update]
async fn test_init() {
    test::init::test_init().await;
}

#[query(composite = true)]
async fn get_item_page_data(
    arg: common::item::ItemPageRequest,
) -> Result<common::item::ItemPageResponse, String> {
    match store::get_item_page_data(arg).await {
        Ok(res) => Ok(res),
        Err(e) => {
            // TODO: Log error
            Err(e.1)
        }
    }
}

#[query(composite = true)]
async fn get_home_page_data(arg: HomePageRequest) -> Result<HomePageResponse, HomePageError> {
    route::home::get_home_page_data(arg).await
}

#[query]
fn get_user_basket_page_data(
    currency: Currency,
) -> Result<
    common::route::basket::UserBasketPageResponse,
    (common::route::basket::UserBasketPageError, String),
> {
    route::basket::get_user_basket_page_data(currency)
}

#[query]
fn get_account_page_data() -> Result<AccountPageResponse, AccountPageError> {
    route::account::get_account_page_data()
}

#[update]
async fn set_user_profile(profile: SetUserProfileRequest) -> Result<(), SetUserProfileError> {
    actor::user::set_user_profile(profile)
}

#[update]
async fn get_trusted_origins() -> Vec<String> {
    vec![
        // Origins should be in the format defined by the Window.postMessage method (https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage#the_dispatched_event)
        String::from("https://anthol.net"),
        String::from("https://www.anthol.net"),
        String::from("https://qqyox-jyaaa-aaaap-abtwa-cai.icp0.io/"),
    ]
}

ic_cdk::export_candid!();
