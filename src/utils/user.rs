use anyhow::Result;
use candid::Principal;

use ic_agent::Agent;

use crate::{
    types::{smart_vault_err::SmartVaultErr, user::{User, AddUserArgs}},
    utils::agent::{make_call_with_agent, CallType},
};

pub async fn delete_user(agent: &Agent, u: Principal) -> anyhow::Result<(), SmartVaultErr> {
    let r: Result<(), SmartVaultErr> = make_call_with_agent(
        agent,
        CallType::Update("delete_user".into()),
        Some(u.as_slice()),
    )
    .await
    .unwrap();

    r
}

pub async fn create_user(agent: &Agent, aua: &AddUserArgs) -> anyhow::Result<User, SmartVaultErr> {
    let user: Result<User, SmartVaultErr> = make_call_with_agent(
        agent,
        CallType::Update("create_user".into()),
        Some(aua),
    )
    .await
    .unwrap();

    user
}
