/*
    Copyright (c) 2026 XBID LABS LLC

    This file is part of XBID-AI project.
    Licensed under the MIT License.
    Author: Fred Kyung-jin Rezeau (오경진 吳景振) <hello@kyungj.in>
*/

use crate::{storage, emission, types::{Error, EmissionParams}};
use soroban_sdk::{token, Address, BytesN, Env};

pub fn initialize(env: &Env, admin: Address, token: Address,
    fund: Address, rewards: Address, ledger: u32) -> Result<(), Error> {
    if storage::has_token(env) {
        return Err(Error::AlreadyInitialized);
    }

    // Provenance requirement checks (addresses).
    if admin == fund || admin == rewards || fund == rewards {
        return Err(Error::InvalidAddress);
    }

    // Provenance requirement checks (supply).
    let params = EmissionParams::default();
    let client = token::Client::new(env, &token);
    let self_addr = env.current_contract_address();
    if client.balance(&admin) != params.total_supply
        || client.balance(&self_addr) != 0 {
        return Err(Error::InvalidSupply);
    }

    admin.require_auth();

    client.transfer(&admin, &env.current_contract_address(), &params.total_supply);
    client.transfer(&env.current_contract_address(), &fund, &params.allocation);

    storage::set_admin(env, &admin);
    storage::set_token(env, &token);
    storage::set_fund(env, &fund);
    storage::set_rewards(env, &rewards);
    storage::set_ledger(env, ledger);
    storage::set_emitted(env, 0);
    storage::extend_ttl(env);

    Ok(())
}

pub fn release(env: &Env) -> i128 {
    let origin = storage::get_ledger(env);
    let target = env.ledger().sequence();
    let realized = storage::get_emitted(env);
    let delta = emission::delta(origin, target, realized);
    if delta == 0 {
        return 0;
    }

    let params = EmissionParams::default();
    let fund = (delta * params.fund_bps) / 10000;
    let rewards = delta - fund;
    let client = token::Client::new(env, &storage::get_token(env));
    client.transfer(
        &env.current_contract_address(),
        &storage::get_fund(env),
        &fund,
    );
    client.transfer(
        &env.current_contract_address(),
        &storage::get_rewards(env),
        &rewards,
    );

    storage::set_emitted(env, realized + delta);
    storage::extend_ttl(env);

    delta
}

pub fn upgrade(env: &Env, hash: BytesN<32>) -> Result<(), Error> {
    let admin = storage::get_admin(env);
    admin.require_auth();

    env.deployer().update_current_contract_wasm(hash);
    storage::extend_ttl(env);

    Ok(())
}