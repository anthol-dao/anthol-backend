use crate::{
    actor::{
        user::{self, basket::Basket, UserActorVersion},
        ActorData,
    },
    item::ItemFluctuateData,
    ACTORS, ITEM_FLUCTUATE_DATA, LOG, MARKETS, STORES,
};
use ic_cdk::api::caller;
use shared::{
    route::basket::{BasketItemError, UserBasketPageError, UserBasketPageResponse},
    unit::Currency,
};

pub fn get_user_basket_page_data(
    currency: Currency,
) -> Result<UserBasketPageResponse, (UserBasketPageError, String)> {
    let caller = caller();

    let actor = ACTORS.with(|actors| actors.borrow().get(&caller.into()));
    let user = match actor {
        Some(actor) => match actor {
            ActorData::User(user) => user,
            _ => {
                return Err((
                    UserBasketPageError::ActorIsNotUser,
                    format!("Actor {} is not user.", caller),
                ))
            }
        },
        None => {
            return Err((
                UserBasketPageError::ActorNotFound,
                format!("Actor {} not found.", caller),
            ))
        }
    };

    let basket = match user.version {
        UserActorVersion::V1 { basket, .. } => basket,
    };

    match basket {
        Basket::V1(basket) => {
            let physical_items =
                get_basket_physical_items_response(currency, basket.physical_items);
            let digital_items = get_basket_digital_items_response(currency, basket.digital_items);

            Ok(UserBasketPageResponse {
                physical_items,
                digital_items,
            })
        }
    }
}

fn get_basket_physical_items_response(
    currency: Currency,
    vec: Vec<user::basket::PhysicalItemGroupOfSender>,
) -> Vec<shared::route::basket::PhysicalItemGroupOfSender> {
    let mut res: Vec<shared::route::basket::PhysicalItemGroupOfSender> = Vec::new();

    for group in vec {
        match group {
            user::basket::PhysicalItemGroupOfSender::V1(group) => {
                STORES.with(|stores| {
                    let store = stores.borrow().get(&group.store_principal);
                    // TODO: handle error if store not found
                    if let Some(store) = store {
                        let items = group
                            .items
                            .iter()
                            .map(|item| {
                                let user::basket::PhysicalItemInBasket::V1(item) = item;
                                let market = MARKETS
                                    .with(|markets| markets.borrow().get(&item.market_principal));
                                let market = match market {
                                    Some(market) => market,
                                    None => return Err((BasketItemError::MarketNotFound, None)),
                                };

                                let (item_id, item_name, price, stock) =
                                    ITEM_FLUCTUATE_DATA.with(|data| {
                                        data.borrow()
                                            .get(&(
                                                group.store_principal,
                                                item.item_key,
                                                item.attr_keys,
                                            ))
                                            .map(|data| match data {
                                                ItemFluctuateData::V1 {
                                                    id,
                                                    name,
                                                    price,
                                                    stock,
                                                    ..
                                                } => (
                                                    id,
                                                    name,
                                                    price
                                                        .get(&currency)
                                                        .copied()
                                                        .unwrap_or_default(),
                                                    stock,
                                                ),
                                            })
                                            .unwrap_or((
                                                Default::default(),
                                                Default::default(),
                                                Default::default(),
                                                Default::default(),
                                            ))
                                        // handle error by returning default values
                                    });

                                Ok(shared::route::basket::PhysicalItemInBasket {
                                    market_id: market.id,
                                    market_name: market.name.clone(),
                                    item_id,
                                    item_name,
                                    image: item.image.clone(),
                                    attr_keys: item.attr_keys,
                                    attrs: item.attrs.clone(),
                                    price,
                                    count: item.count,
                                    stock,
                                })
                            })
                            .collect::<Vec<
                                Result<
                                    shared::route::basket::PhysicalItemInBasket,
                                    (
                                        BasketItemError,
                                        Option<shared::route::basket::PhysicalItemInBasket>,
                                    ),
                                >,
                            >>();

                        res.push(shared::route::basket::PhysicalItemGroupOfSender {
                            store_id: store.id,
                            store_name: store.name.clone(),
                            items,
                        });
                    }
                });
            }
        }
    }

    res
}

fn get_basket_digital_items_response(
    currency: Currency,
    vec: Vec<user::basket::DigitalItemInBasket>,
) -> Vec<
    Result<
        shared::route::basket::DigitalItemInBasket,
        (
            BasketItemError,
            Option<shared::route::basket::DigitalItemInBasket>,
        ),
    >,
> {
    vec![]
}
