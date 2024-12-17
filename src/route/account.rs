use crate::{
    actor::{
        user::{UserActorVersion, UserProfileV1, UserProfileVersion},
        ActorData,
    },
    ACTORS,
};
use ic_cdk::api;
use common::actor::account::{AccountPageError, AccountPageResponse, UserAccountPageResponse};

pub fn get_account_page_data() -> Result<AccountPageResponse, AccountPageError> {
    let caller = api::caller();
    let actor = match ACTORS.with(|actors| actors.borrow().get(&caller.into())) {
        Some(actor) => actor,
        None => return Err(AccountPageError::AccountNotFound(caller)),
    };

    match actor {
        ActorData::User(user) => match user.version {
            UserActorVersion::V1 {
                profile:
                    UserProfileVersion::V1(UserProfileV1 {
                        name,
                        birth_name,
                        mail_address,
                        image,
                    }),
                ..
            } => Ok(AccountPageResponse::User(UserAccountPageResponse {
                name,
                birth_name,
                mail_address,
                image,
            })),
        },
        _ => Err(AccountPageError::AccountNotFound(caller)),
    }
}
