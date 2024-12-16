use crate::{
    actor::{user::UserActorVersion, ActorData},
    market::get_markets_with_recent_item_glances,
    ACTORS,
};
use ic_cdk::api;
use shared::{
    market::MarketPrincipal,
    route::home::{HomePageError, HomePageRequest, HomePageResponse},
};

pub async fn get_home_page_data(arg: HomePageRequest) -> Result<HomePageResponse, HomePageError> {
    let caller = api::caller();
    let actor = match ACTORS.with(|actors| actors.borrow().get(&caller.into())) {
        Some(actor) => actor,
        None => return Err(HomePageError::AccountNotFound(caller)),
    };

    let result = match actor {
        ActorData::User(user) => match user.version {
            UserActorVersion::V1 {
                following_markets,
                favorite_markets,
                ..
            } => {
                let len = following_markets.len().min(5);
                let first_5_market_ids: Vec<MarketPrincipal> = following_markets
                    .iter()
                    .take(len)
                    .map(|market| *market.0)
                    .collect();

                let markets =
                    get_markets_with_recent_item_glances(first_5_market_ids, arg.currency).await;

                HomePageResponse { markets }
            }
        },
        _ => return Err(HomePageError::AccountNotFound(caller)),
    };

    Ok(result)
}
