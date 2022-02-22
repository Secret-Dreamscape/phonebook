use cosmwasm_std::{
  debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse,
  Querier, StdError, StdResult, Storage,
};
use secret_toolkit::utils::InitCallback;

use crate::msg::{GameContractInitMsg, HandleMsg, InitMsg, ListResponse, QueryMsg};
use crate::state::{config, config_read, Contract, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  msg: InitMsg,
) -> StdResult<InitResponse> {
  let state = State {
    admin: env.message.sender.clone(),
    contracts: vec![],
    remove_after: 2 * 60 * 60,
    stamp_addr: msg.stamp_addr,
    stamp_hash: msg.stamp_hash,
    game_code_id: msg.game_code_id,
    game_hash: msg.game_hash,
    jackpot_addr: msg.jackpot_addr,
    jackpot_hash: msg.jackpot_hash,
  };

  config(&mut deps.storage).save(&state)?;

  debug_print!("Contract was initialized by {}", env.message.sender);
  Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  msg: HandleMsg,
) -> StdResult<HandleResponse> {
  match msg {
    HandleMsg::Refresh {} => try_refresh(deps, env),
    HandleMsg::CreateNewTable {
      label,
      password,
      min_buy,
      max_buy,
    } => try_create_new_table(deps, env, label, password, min_buy, max_buy),
    HandleMsg::UpdateRemoveTimeout { new_timeout } => {
      try_update_removal_timeout(deps, env, new_timeout)
    }
    HandleMsg::UpdateValidCodeId {
      new_code_id,
      new_hash,
    } => try_update_valid_code_id(deps, env, new_code_id, new_hash),
    HandleMsg::UpdateStamper {
      new_stamp_addr,
      new_stamp_hash,
    } => try_update_stamper(deps, env, new_stamp_addr, new_stamp_hash),
    HandleMsg::UpdateJackpot {
      new_jackpot_addr,
      new_jackpot_hash,
    } => try_update_jackpot(deps, env, new_jackpot_addr, new_jackpot_hash),
    HandleMsg::PassTheHatOn { new_admin } => try_passing_the_admin_hat(deps, env, new_admin),
    HandleMsg::RegisteredCallback {
      address,
      private,
      label,
      referrer,
    } => try_register_callback(deps, env, address, private, label, referrer),
  }
}

fn try_update_jackpot<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  new_jackpot_addr: HumanAddr,
  new_jackpot_hash: String,
) -> Result<HandleResponse, StdError> {
  let mut state = config(&mut deps.storage).load()?;
  if state.admin != env.message.sender {
    return Err(StdError::generic_err(
      "Only the admin can update the jackpot address",
    ));
  }
  state.jackpot_addr = new_jackpot_addr;
  state.jackpot_hash = new_jackpot_hash;
  config(&mut deps.storage).save(&state)?;
  Ok(HandleResponse::default())
}

fn try_register_callback<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  address: HumanAddr,
  private: bool,
  label: String,
  referrer: String,
) -> Result<HandleResponse, StdError> {
  let mut state = config(&mut deps.storage).load()?;

  if referrer != env.contract_code_hash {
    // TODO: come up with a better way to do this, perhaps a unique rolling identifier that changes every time a new game contract is instantiated
    return Err(StdError::generic_err("referrer is not the contract hash"));
  }

  state.contracts.push(Contract {
    address,
    private,
    created_at: env.block.time,
    label,
  });
  config(&mut deps.storage).save(&state)?;
  clean_old_contracts(&mut state, &env);
  debug_print!("contracts count = {}", state.contracts.len());
  Ok(HandleResponse::default())
}

fn clean_old_contracts(state: &mut State, env: &Env) {
  // TODO: the code below works on everything except cosmwasm for some bizarre reason, this needs to be fixed because at some point it will go over the limit of what a block can have and we will need an additional block
  let remove_after = state.remove_after;
  let mut contracts = state.contracts.clone();
  contracts.retain(|contract| contract.created_at + remove_after > env.block.time);
  state.contracts = contracts;
}

fn try_refresh<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
) -> StdResult<HandleResponse> {
  config(&mut deps.storage).update(|mut state| {
    clean_old_contracts(&mut state, &env);
    debug_print!("new contracts count = {}", state.contracts.len());
    Ok(state)
  })?;

  debug_print!("Contracts refreshed successfully");
  Ok(HandleResponse::default())
}

fn try_create_new_table<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  label: String,
  password: Option<String>,
  min_buy: u64,
  max_buy: u64,
) -> StdResult<HandleResponse> {
  let state = config(&mut deps.storage).load()?;

  let new_game_instantiation = GameContractInitMsg {
    bg: 0,
    password,
    label: label.clone(),
    stamp_addr: state.stamp_addr.clone(),
    stamp_hash: state.stamp_hash.clone(),
    callback_addr: env.contract.address.clone(),
    callback_hash: env.contract_code_hash.clone(),
    min_buy,
    max_buy,
    jackpot_addr: state.jackpot_addr.clone(),
    jackpot_hash: state.jackpot_hash.clone(),
  };

  let new_game_msg = new_game_instantiation.to_cosmos_msg(
    label.clone(),
    state.game_code_id,
    state.game_hash.clone(),
    None,
  )?;

  debug_print!("Contract registered successfully");
  Ok(HandleResponse {
    messages: vec![new_game_msg],
    log: vec![],
    data: None,
  })
}

fn try_update_removal_timeout<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  new_timeout: u64,
) -> StdResult<HandleResponse> {
  config(&mut deps.storage).update(|mut state| {
    require_admin(env, &mut state)?;
    state.remove_after = new_timeout;
    debug_print!("new contracts timeout = {}", new_timeout);
    Ok(state)
  })?;

  debug_print!("Contracts timeout updated successfully");
  Ok(HandleResponse::default())
}

fn try_update_stamper<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  new_stamp_addr: HumanAddr,
  new_stamp_hash: String,
) -> Result<HandleResponse, StdError> {
  config(&mut deps.storage).update(|mut state| {
    require_admin(env, &mut state)?;
    state.stamp_addr = new_stamp_addr.clone();
    state.game_hash = new_stamp_hash;
    state.contracts.clear();
    debug_print!("new stamper addr = {}", new_stamp_addr.clone());
    Ok(state)
  })?;

  debug_print!("Stamper contract updated successfully");
  Ok(HandleResponse::default())
}

fn try_update_valid_code_id<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  new_code_id: u64,
  new_hash: String,
) -> StdResult<HandleResponse> {
  config(&mut deps.storage).update(|mut state| {
    require_admin(env, &mut state)?;
    state.game_code_id = new_code_id;
    state.game_hash = new_hash;
    state.contracts.clear();
    debug_print!("new valid code id = {}", new_code_id);
    Ok(state)
  })?;

  debug_print!("Valid code id updated successfully");
  Ok(HandleResponse::default())
}

fn try_passing_the_admin_hat<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  new_hat: HumanAddr,
) -> StdResult<HandleResponse> {
  config(&mut deps.storage).update(|mut state| {
    require_admin(env, &mut state)?;
    state.admin = new_hat.clone();
    debug_print!("new admin = {}", new_hat);
    Ok(state)
  })?;

  debug_print!("Admin hat passed on successfully, farewell soldier");
  Ok(HandleResponse::default())
}

fn require_admin(env: Env, state: &mut State) -> StdResult<bool> {
  if env.message.sender != state.admin {
    return Err(StdError::generic_err("Only the contract admin can do this"));
  }
  Ok(true)
}

pub fn query<S: Storage, A: Api, Q: Querier>(
  deps: &Extern<S, A, Q>,
  msg: QueryMsg,
) -> StdResult<Binary> {
  match msg {
    QueryMsg::GetList {} => to_binary(&query_list(deps)?),
  }
}

fn query_list<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<ListResponse> {
  let state = config_read(&deps.storage).load()?;
  Ok(ListResponse {
    contracts: state.contracts,
  })
}
