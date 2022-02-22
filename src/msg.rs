use crate::state::Contract;
use cosmwasm_std::HumanAddr;
use schemars::JsonSchema;
use secret_toolkit::utils::InitCallback;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
  pub stamp_addr: HumanAddr,
  pub stamp_hash: String,
  pub game_code_id: u64,
  pub game_hash: String,
  pub jackpot_addr: HumanAddr,
  pub jackpot_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
  CreateNewTable {
    label: String,
    password: Option<String>,
    min_buy: u64,
    max_buy: u64,
  },
  Refresh {},
  UpdateRemoveTimeout {
    new_timeout: u64,
  },
  UpdateJackpot {
    new_jackpot_addr: HumanAddr,
    new_jackpot_hash: String,
  },
  UpdateStamper {
    new_stamp_addr: HumanAddr,
    new_stamp_hash: String,
  },
  UpdateValidCodeId {
    new_code_id: u64,
    new_hash: String,
  },
  PassTheHatOn {
    new_admin: HumanAddr,
  },
  RegisteredCallback {
    address: HumanAddr,
    private: bool,
    label: String,
    referrer: String,
  },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  GetList {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListResponse {
  pub contracts: Vec<Contract>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GameContractInitMsg {
  pub bg: u64,
  pub password: Option<String>,
  pub label: String,
  pub stamp_addr: HumanAddr,
  pub stamp_hash: String,
  pub callback_addr: HumanAddr,
  pub callback_hash: String,
  pub min_buy: u64,
  pub max_buy: u64,
  pub jackpot_addr: HumanAddr,
  pub jackpot_hash: String,
}

impl InitCallback for GameContractInitMsg {
  const BLOCK_SIZE: usize = 256;
}
