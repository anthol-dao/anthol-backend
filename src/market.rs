use crate::{MARKETS, MARKETS_IN_ID};
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
use common::{
    item::{ItemDataInMarket, ItemId},
    market::{
        MarketDataResponseWithItemGlances, MarketId, MarketInitArg, MarketName, MarketPrincipal,
    },
    store::StoreId,
    unit::Currency,
    util::scale::u128::TRILLION,
};
use std::borrow::Cow;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct MarketData {
    pub id: MarketId,
    pub name: MarketName,
    pub details: MarketDetails,
}

impl Storable for MarketData {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum MarketDetails {
    None,
}

pub async fn get_markets_with_recent_item_glances(
    markets: Vec<MarketPrincipal>,
    currency: Currency,
) -> Vec<MarketDataResponseWithItemGlances> {
    use futures::future::join_all;

    let futures = markets.into_iter().map(|market_canister| async move {
        match get_recent_item_glances(market_canister, currency).await {
            Ok(each_res) => Some(each_res),
            Err(e) => {
                ic_cdk::println!("Error: {:?} on get_recent_item_glances", e);
                None
            }
        }
    });

    let res = join_all(futures).await;

    res.into_iter().flatten().collect()
}

pub async fn get_recent_item_glances(
    canister_id: MarketPrincipal,
    currency: Currency,
) -> CallResult<MarketDataResponseWithItemGlances> {
    let (res,) = call(
        canister_id.into(),
        "get_recent_item_glances",
        (caller(), currency),
    )
    .await?;
    Ok(res)
}

pub async fn insert_items(
    canister_id: &MarketPrincipal,
    vec: Vec<((StoreId, ItemId), ItemDataInMarket)>,
) -> CallResult<()> {
    call(
        (*canister_id).into(),
        "insert_items_to_market",
        (caller(), vec),
    )
    .await
}

pub async fn update_market_data(
    canister_id: &MarketPrincipal,
    id: &MarketId,
    name: &MarketName,
) -> CallResult<()> {
    call(
        (*canister_id).into(),
        "update_market_data",
        (caller(), id, name),
    )
    .await
}

pub async fn create_market_canister(
    id: &MarketId,
    name: &MarketName,
) -> Result<MarketPrincipal, CreateMarketError> {
    MARKETS_IN_ID.with(|stores| {
        if stores.borrow().contains_key(id) {
            return Err(CreateMarketError(
                CreateMarketRejectionCode::MarketAlreadyExists,
                format!("Market with id {} already exists", id),
            ));
        }
        Ok(())
    })?;

    let canister_id = create_canister(CreateCanisterArgument::default(), TRILLION)
        .await?
        .0
        .canister_id;

    let wasm_module =
        include_bytes!("../../../target/wasm32-unknown-unknown/release/anthol_market.wasm")
            .to_vec();

    let arg = Some(MarketInitArg {
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

    let canister_id: MarketPrincipal = canister_id.into();

    MARKETS.with(|stores| {
        stores.borrow_mut().insert(
            canister_id,
            MarketData {
                id: *id,
                name: name.clone(),
                details: MarketDetails::None,
            },
        );
    });

    MARKETS_IN_ID.with(|stores| {
        stores.borrow_mut().insert(*id, canister_id);
    });

    Ok(canister_id)
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub struct CreateMarketError(CreateMarketRejectionCode, String);

#[derive(CandidType, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub enum CreateMarketRejectionCode {
    MarketAlreadyExists,
    CallRejectionCode(RejectionCode),
}

impl From<(RejectionCode, String)> for CreateMarketError {
    fn from((code, msg): (RejectionCode, String)) -> Self {
        CreateMarketError(CreateMarketRejectionCode::CallRejectionCode(code), msg)
    }
}
