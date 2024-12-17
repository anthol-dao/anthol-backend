use crate::{
    actor::{
        user::{
            basket::Basket, DataAboutFollowingMarketV1, UserActor, UserActorVersion, UserProfileV1,
            UserProfileVersion,
        },
        ActorData,
    },
    market, store, StoreId, TagData, ACTORS, TAGS,
};
use candid::Principal;
use ic_cdk::api;
use common::{actor::account::AccountImage, item::Tag, market::MarketPrincipal, store::StoreName, util::network::{dfx_network, DfxNetwork}};
use std::collections::BTreeMap;

mod item_init;
mod market_init;

pub async fn test_init() {
    match dfx_network() {
        DfxNetwork::Local => test_init_core().await,
        _ => api::print("Running in production environment, skipping test_init."),
    }
}

async fn test_init_core() {
    use futures::future::join_all;

    let items = item_init::get();

    let stores_init: Vec<(StoreId, StoreName)> = vec![(
        "specimens".try_into().unwrap(),
        "Specimens".try_into().unwrap(),
    )];

    for store in stores_init {
        let res = store::create_store_canister(&store.0, &store.1)
            .await
            .unwrap();

        let _ = store::insert_items(res, items.clone()).await;
    }

    let mut following_markets = BTreeMap::<MarketPrincipal, DataAboutFollowingMarketV1>::new();
    let mut favorite_markets = Vec::<MarketPrincipal>::new();

    let futures = market_init::markets_init()
        .into_iter()
        .map(|market| async move {
            let canister_id = market::create_market_canister(&market.0, &market.1)
                .await
                .unwrap();

            let _ = market::insert_items(&canister_id, market.2).await;

            (
                canister_id,
                DataAboutFollowingMarketV1 {
                    followed_time: ic_cdk::api::time(),
                    last_bought_time: None,
                },
            )
        });

    join_all(futures).await.into_iter().for_each(|(k, v)| {
        following_markets.insert(k, v);
        favorite_markets.push(k);
    });

    let tags = [
        (
            "camera".to_string(),
            TagData {
                tagged_items: vec![1],
                tagged_markets: vec![],
            },
        ),
        (
            "polaroid".to_string(),
            TagData {
                tagged_items: vec![1],
                tagged_markets: vec![],
            },
        ),
        (
            "cap".to_string(),
            TagData {
                tagged_items: vec![2],
                tagged_markets: vec![],
            },
        ),
        (
            "furniture".to_string(),
            TagData {
                tagged_items: vec![2],
                tagged_markets: vec![],
            },
        ),
        (
            "pen".to_string(),
            TagData {
                tagged_items: vec![3],
                tagged_markets: vec![],
            },
        ),
    ];

    ACTORS.with(|p| {
        p.borrow_mut().insert(
            Principal::anonymous().into(),
            ActorData::User(UserActor {
                version: UserActorVersion::V1 {
                    profile: UserProfileVersion::V1(UserProfileV1 {
                        name: "Test User".to_string(),
                        birth_name: "".to_string(),
                        mail_address: "".to_string(),
                        image: AccountImage::None,
                    }),
                    following_markets,
                    favorite_markets,
                    favorite_stores: vec![],
                    favorite_items: vec![],
                    basket: Basket::default(),
                },
            }),
        )
    });

    TAGS.with(|p| {
        let mut tags_map = p.borrow_mut();
        tags.iter().for_each(|(name, data)| {
            tags_map.insert(Tag::new(name).unwrap(), data.clone());
        })
    });
}
