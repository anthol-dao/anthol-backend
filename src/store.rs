use crate::{MARKETS, MARKETS_IN_ID, STORES, STORES_IN_ID};
use anthol_store::item::Item;
use candid::{CandidType, Decode, Deserialize, Encode};
use ic_cdk::api::{
    call::{call, CallResult, RejectionCode},
    caller,
    management_canister::main::{
        create_canister, install_code, CanisterInstallMode, CreateCanisterArgument,
        InstallCodeArgument,
    },
};
use ic_stable_structures::{storable::Bound, Storable};
use shared::{
    item::{
        get_item_page_response, ItemId, ItemKey, ItemPageError, ItemPageErrorCode,
        ItemPageFromStoreErrorCode, ItemPageRequest, ItemPageResponse,
        ItemPageResponseFromStoreCanister,
    },
    store::{StoreId, StoreInitArg, StoreName, StorePrincipal},
    util::scale::u128::TRILLION,
};
use std::borrow::Cow;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct StoreData {
    pub id: StoreId,
    pub name: StoreName,
    pub details: StoreDetails,
}

impl Storable for StoreData {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum StoreDetails {
    None,
}

pub async fn update_store_data(
    canister_id: &StorePrincipal,
    id: &StoreId,
    name: &StoreName,
) -> CallResult<()> {
    call(
        (*canister_id).into(),
        "update_store_data",
        (caller(), id, name),
    )
    .await
}

pub async fn create_store_canister(
    id: &StoreId,
    name: &StoreName,
) -> Result<StorePrincipal, CreateStoreError> {
    STORES_IN_ID.with(|stores| {
        if stores.borrow().contains_key(id) {
            return Err(CreateStoreError(
                CreateStoreRejectionCode::StoreAlreadyExists,
                format!("Store with id {} already exists", id),
            ));
        }
        Ok(())
    })?;

    let canister_id = create_canister(CreateCanisterArgument::default(), TRILLION)
        .await?
        .0
        .canister_id;

    let wasm_module =
        include_bytes!("../../../target/wasm32-unknown-unknown/release/anthol_store.wasm").to_vec();

    let arg = Some(StoreInitArg {
        id: *id,
        name: name.clone(),
    });

    install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module,
        arg: Encode!(&arg).unwrap(),
    })
    .await?;

    let canister_id: StorePrincipal = canister_id.into();

    STORES.with_borrow_mut(|stores| {
        stores.insert(
            canister_id,
            StoreData {
                id: *id,
                name: name.clone(),
                details: StoreDetails::None,
            },
        );
    });

    STORES_IN_ID.with_borrow_mut(|stores| {
        stores.insert(*id, canister_id);
    });

    Ok(canister_id)
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub struct CreateStoreError(CreateStoreRejectionCode, String);

#[derive(CandidType, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub enum CreateStoreRejectionCode {
    StoreAlreadyExists,
    CallRejectionCode(RejectionCode),
}

impl From<(RejectionCode, String)> for CreateStoreError {
    fn from((code, msg): (RejectionCode, String)) -> Self {
        CreateStoreError(CreateStoreRejectionCode::CallRejectionCode(code), msg)
    }
}

pub async fn get_item_page_data(arg: ItemPageRequest) -> Result<ItemPageResponse, ItemPageError> {
    let market_canister = MARKETS_IN_ID
        .with(|markets| markets.borrow().get(&arg.market_id))
        .ok_or(ItemPageError(
            ItemPageErrorCode::MarketNotFound,
            format!("Market with id {} not found", arg.market_id),
        ))?;

    let market_name = MARKETS
        .with(|markets| {
            markets
                .borrow()
                .get(&market_canister)
                .map(|market| market.name.clone())
        })
        .ok_or(ItemPageError(
            ItemPageErrorCode::MarketNotFound,
            format!("Market with id {} not found", arg.market_id),
        ))?;

    let store_canister = STORES_IN_ID
        .with(|stores| stores.borrow().get(&arg.store_id))
        .ok_or(ItemPageError(
            ItemPageErrorCode::StoreNotFound,
            format!("Store with id {} not found", arg.store_id),
        ))?;

    let (res,): (Result<ItemPageResponseFromStoreCanister, (ItemPageFromStoreErrorCode, String)>,) =
        call(
            store_canister.into(),
            "get_item_page_data_from_store",
            (caller(), arg.request_to_store),
        )
        .await?;

    let res = get_item_page_response(res?, market_name);

    Ok(res)
}

pub async fn insert_items(
    canister_id: StorePrincipal,
    vec: Vec<Item>,
) -> CallResult<Vec<(ItemId, ItemKey)>> {
    let (res,) = call(canister_id.into(), "insert_items_to_store", (caller(), vec)).await?;
    Ok(res)
}
