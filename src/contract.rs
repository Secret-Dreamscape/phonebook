use cosmwasm_std::{debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError, StdResult, Storage, HumanAddr};

use crate::msg::{HandleMsg, InitMsg, ListResponse, QueryMsg};
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
    valid_code_id: msg.valid_code_id
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
    HandleMsg::Register { address, label, private } => try_register(deps, env, address, label, private),
    HandleMsg::UpdateRemoveTimeout { new_timeout } => try_update_removal_timeout(deps, env, new_timeout),
    HandleMsg::UpdateValidCodeId { new_code_id } => try_update_valid_code_id(deps, env, new_code_id),
    HandleMsg::PassTheHatOn { new_admin } => try_passing_the_admin_hat(deps, env, new_admin),
  }
}

fn clean_old_contracts(state : &mut State, env: &Env) {
  // TODO: the code below works on everything except cosmwasm for some bizarre reason, this needs to be fixed because at some point it will go over the limit of what a block can have and we will need an additional block
  // let remove_after = state.remove_after;
  // state.contracts.retain(|x| (x.created_at - env.block.time) < remove_after);
}

fn try_refresh<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>, env: Env) -> StdResult<HandleResponse> {
  config(&mut deps.storage).update(|mut state| {
    clean_old_contracts(&mut state, &env);
    debug_print!("new contracts count = {}", state.contracts.len());
    Ok(state)
  })?;

  debug_print!("Contracts refreshed successfully");
  Ok(HandleResponse::default())
}

fn try_register<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>, env: Env, address: HumanAddr, label: String, private: bool) -> StdResult<HandleResponse> {
  config(&mut deps.storage).update(|mut state| {
    // FIXME: Add security check to make sure that the contract being added has the correct codeId, secret-toolkit doesn't provide a way to do this as far as I know, so this will be complicated
    state.contracts.push(Contract {
      address,
      label,
      private,
      created_at: env.block.time
    });
    clean_old_contracts(&mut state, &env);
    debug_print!("contracts count = {}", state.contracts.len());
    Ok(state)
  })?;

  debug_print!("Contract registered successfully");
  Ok(HandleResponse::default())
}

fn try_update_removal_timeout<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>, env: Env, new_timeout: u64) -> StdResult<HandleResponse> {
  config(&mut deps.storage).update(|mut state| {
    require_admin(env, &mut state)?;
    state.remove_after = new_timeout;
    debug_print!("new contracts timeout = {}", new_timeout);
    Ok(state)
  })?;

  debug_print!("Contracts timeout updated successfully");
  Ok(HandleResponse::default())
}

fn try_update_valid_code_id<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>, env: Env, new_code_id: u64) -> StdResult<HandleResponse> {
  config(&mut deps.storage).update(|mut state| {
    require_admin(env, &mut state)?;
    state.valid_code_id = new_code_id;
    state.contracts.clear();
    debug_print!("new valid code id = {}", new_code_id);
    Ok(state)
  })?;

  debug_print!("Valid code id updated successfully");
  Ok(HandleResponse::default())
}

fn try_passing_the_admin_hat<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>, env: Env, new_hat: HumanAddr) -> StdResult<HandleResponse> {
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
  Ok(ListResponse { contracts: state.contracts })
}
