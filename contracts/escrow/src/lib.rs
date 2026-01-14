/*
    Copyright (c) 2026 XBID LABS LLC

    This file is part of XBID-AI project.
    Licensed under the MIT License.
    Author: Fred Kyung-jin Rezeau (오경진 吳景振) <hello@kyungj.in>
*/

#![no_std]

mod escrow;
mod storage;
mod emission;
mod types;

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};
use types::Error;

#[contract]
pub struct Escrow;

#[contractimpl]
impl Escrow {
    pub fn initialize(env: Env, admin: Address, token: Address,
        fund: Address, rewards: Address, ledger: u32) -> Result<(), Error> {
        escrow::initialize(&env, admin, token, fund, rewards, ledger)
    }

    pub fn release(env: Env) -> i128 {
        escrow::release(&env)
    }

    pub fn upgrade(env: Env, hash: BytesN<32>) -> Result<(), Error> {
        escrow::upgrade(&env, hash)
    }
}

#[cfg(test)]
mod test;