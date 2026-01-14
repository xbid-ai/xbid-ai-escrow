/*
    Copyright (c) 2026 XBID LABS LLC

    This file is part of XBID-AI project.
    Licensed under the MIT License.
    Author: Fred Kyung-jin Rezeau (오경진 吳景振) <hello@kyungj.in>
*/

use soroban_sdk::{contracterror, contracttype};

#[derive(Clone, Copy)]
pub struct EmissionParams {
    pub total_supply: i128,
    pub allocation: i128,
    pub fund_bps: i128,
    pub half_life: i128,
}

impl EmissionParams {
    pub const fn default() -> Self {
        Self {
            total_supply: 10_000_000_070_000_000,
            allocation: 200_000_001_400_000,
            fund_bps: 100,
            half_life: 7 * 6_307_200,
        }
    }

    pub const fn pool(&self) -> i128 {
        self.total_supply - self.allocation
    }
}

#[derive(Clone, Copy)]
#[contracterror]
pub enum Error {
    AlreadyInitialized = 1,
    InvalidAddress = 2,
    InvalidSupply = 3,
}

#[derive(Clone, Copy)]
#[contracttype]
pub enum Storage {
    Admin,
    Token,
    Fund,
    Rewards,
    Ledger,
    Emitted,
}