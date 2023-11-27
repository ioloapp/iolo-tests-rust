use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Debug, CandidType, Deserialize, Serialize, Clone)]
pub struct UUID(pub u128);
pub type UserVaultID = UUID;

#[derive(Debug, CandidType, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Principal,
    pub name: Option<String>,
    pub email: Option<String>,
    pub user_type: Option<UserType>,
    pub date_created: u64,
    pub date_modified: u64,
    pub date_last_login: Option<u64>,
    pub user_vault_id: Option<UserVaultID>
}

#[derive(Debug, CandidType, Deserialize, Serialize, Clone, PartialEq)]
pub struct AddUserArgs {
    pub id: Principal,
    pub name: Option<String>,
    pub email: Option<String>,
    pub user_type: Option<UserType>,
}

#[derive(Debug, CandidType, Deserialize, Serialize, Clone, PartialEq)]
pub enum UserType {
    Person,
    Company,
}
