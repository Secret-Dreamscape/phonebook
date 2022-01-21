use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{HumanAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
  pub remove_after: u64,
  pub admin: HumanAddr,
  pub contracts: Vec<Contract>,
  pub valid_code_id: u64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Contract {
  pub address: HumanAddr,
  pub label: String,
  pub private: bool,
  pub created_at: u64,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
  singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
  singleton_read(storage, CONFIG_KEY)
}
