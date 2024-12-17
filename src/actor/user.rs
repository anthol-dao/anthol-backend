use super::ActorData;
use crate::ACTORS;
use candid::{CandidType, Deserialize};
use ic_cdk::api;
use common::{
    actor::account::{AccountImage, SetUserProfileError, SetUserProfileRequest},
    item::ItemKey,
    market::MarketPrincipal,
    store::StorePrincipal,
};
use std::{collections::BTreeMap, mem};

pub mod basket;

use basket::Basket;

nest! {
    #[derive(CandidType, Deserialize, Debug, Clone)]
    pub struct UserActor {
        #>[derive(CandidType, Deserialize, Debug, Clone)]
        pub version: pub enum UserActorVersion {
            V1 {
                profile: UserProfileVersion,
                following_markets: BTreeMap<MarketPrincipal, DataAboutFollowingMarketV1>,
                favorite_markets: Vec<MarketPrincipal>,
                favorite_stores: Vec<StorePrincipal>,
                favorite_items: Vec<(StorePrincipal, ItemKey)>,
                basket: Basket,
            },
        },
    }
}

nest! {
    #[derive(CandidType, Deserialize, Debug, Clone, PartialEq)]
    pub enum UserProfileVersion {
        V1(
            #>[derive(CandidType, Deserialize, Debug, Clone, PartialEq)]
            pub struct UserProfileV1 {
                pub name: String,
                pub birth_name: String,
                pub mail_address: String,
                pub image: AccountImage,
            }
        ),
    }
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct DataAboutFollowingMarketV1 {
    pub followed_time: u64,
    pub last_bought_time: Option<u64>,
}

impl DataAboutFollowingMarketV1 {
    pub fn builder() -> DataAboutFollowingMarketV1Builder {
        DataAboutFollowingMarketV1Builder {
            followed_time: None,
            last_bought_time: None,
        }
    }
}

pub struct DataAboutFollowingMarketV1Builder {
    followed_time: Option<u64>,
    last_bought_time: Option<u64>,
}

impl DataAboutFollowingMarketV1Builder {
    pub fn followed_time(mut self, followed_time: u64) -> Self {
        self.followed_time = Some(followed_time);
        self
    }

    pub fn last_bought_time(mut self, last_bought_time: Option<u64>) -> Self {
        self.last_bought_time = last_bought_time;
        self
    }

    pub fn build(self) -> DataAboutFollowingMarketV1 {
        DataAboutFollowingMarketV1 {
            followed_time: self.followed_time.expect("followed_time is required"),
            last_bought_time: self.last_bought_time,
        }
    }
}

pub fn set_user_profile(request: SetUserProfileRequest) -> Result<(), SetUserProfileError> {
    let caller = api::caller();

    let mut actor_borrowed = ACTORS.with(|actors| actors.borrow().get(&caller.into()));
    if let Some(actor) = actor_borrowed {
        if let ActorData::User(mut user) = actor {
            match &mut user.version {
                UserActorVersion::V1 {
                    profile,
                    ref mut following_markets,
                    ref mut favorite_markets,
                    ref mut favorite_stores,
                    ref mut favorite_items,
                    ref mut basket,
                    ..
                } => {
                    let new_profile = UserProfileVersion::V1(UserProfileV1 {
                        name: request.name,
                        birth_name: request.birth_name,
                        mail_address: request.mail_address,
                        image: request.image,
                    });

                    if profile != &new_profile {
                        // Move existing values to a new instance
                        actor_borrowed = Some(ActorData::User(UserActor {
                            version: UserActorVersion::V1 {
                                profile: new_profile,
                                // Reusing existing values with move semantics
                                following_markets: mem::take(following_markets),
                                favorite_markets: mem::take(favorite_markets),
                                favorite_stores: mem::take(favorite_stores),
                                favorite_items: mem::take(favorite_items),
                                basket: mem::take(basket),
                            },
                        }));

                        let _ = ACTORS.with(|actors| {
                            actors
                                .borrow_mut()
                                .insert(caller.into(), actor_borrowed.unwrap())
                        });
                    }
                }
            }
        } else {
            return Err(SetUserProfileError::AccountIsNotUser(caller));
        }
    } else {
        return Err(SetUserProfileError::AccountNotFound(caller));
    }

    Ok(())
}
