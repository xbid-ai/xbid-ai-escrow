/*
    Copyright (c) 2026 XBID LABS LLC

    This file is part of XBID-AI project.
    Licensed under the MIT License.
    Author: Fred Kyung-jin Rezeau (오경진 吳景振) <hello@kyungj.in>
*/

use crate::types::Storage;
use soroban_sdk::{Address, Env};

pub fn get_admin(env: &Env) -> Address {
    env.storage().instance()
        .get::<Storage, Address>(&Storage::Admin)
        .unwrap_or_else(|| panic!("admin not set"))
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance()
        .set::<Storage, Address>(&Storage::Admin, admin);
}

pub fn has_token(env: &Env) -> bool {
    env.storage().instance().has(&Storage::Token)
}

pub fn get_token(env: &Env) -> Address {
    env.storage().instance()
        .get::<Storage, Address>(&Storage::Token)
        .unwrap_or_else(|| panic!("token not set"))
}

pub fn set_token(env: &Env, token: &Address) {
    env.storage().instance()
        .set::<Storage, Address>(&Storage::Token, token);
}

pub fn get_fund(env: &Env) -> Address {
    env.storage().instance()
        .get::<Storage, Address>(&Storage::Fund)
        .unwrap_or_else(|| panic!("fund not set"))
}

pub fn set_fund(env: &Env, fund: &Address) {
    env.storage().instance()
        .set::<Storage, Address>(&Storage::Fund, fund);
}

pub fn get_rewards(env: &Env) -> Address {
    env.storage().instance()
        .get::<Storage, Address>(&Storage::Rewards)
        .unwrap_or_else(|| panic!("rewards not set"))
}

pub fn set_rewards(env: &Env, rewards: &Address) {
    env.storage().instance()
        .set::<Storage, Address>(&Storage::Rewards, rewards);
}

pub fn get_ledger(env: &Env) -> u32 {
    env.storage().instance()
        .get::<Storage, u32>(&Storage::Ledger)
        .unwrap_or_else(|| panic!("ledger not set"))
}

pub fn set_ledger(env: &Env, start: u32) {
    env.storage().instance()
        .set::<Storage, u32>(&Storage::Ledger, &start);
}

pub fn get_emitted(env: &Env) -> i128 {
    env.storage().instance()
        .get::<Storage, i128>(&Storage::Emitted)
        .unwrap_or_else(|| panic!("emitted not set"))
}

pub fn set_emitted(env: &Env, emitted: i128) {
    env.storage().instance()
        .set::<Storage, i128>(&Storage::Emitted, &emitted);
}

pub fn extend_ttl(env: &Env) {
    let max_ttl = env.storage().max_ttl();
    let threshold = max_ttl.saturating_sub(120_960);
    env.storage().instance().extend_ttl(threshold, max_ttl);
}