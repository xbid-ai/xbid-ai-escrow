/*
    Copyright (c) 2026 XBID LABS LLC

    This file is part of XBID-AI project.
    Licensed under the MIT License.
    Author: Fred Kyung-jin Rezeau (오경진 吳景振) <hello@kyungj.in>
*/

use crate::types::EmissionParams;

// Curve: Δ = F(target) − realized.
pub fn delta(origin: u32, target: u32, realized: i128) -> i128 {
    if target <= origin { // Never emit backward.
        return 0;
    }

    let params = EmissionParams::default();
    let pool = params.pool();
    let elapsed = (target - origin) as i128;
    let delta = (pool - ((pool * params.half_life)
        / (params.half_life + elapsed))) - realized;
    if delta <= 0 {
        return 0;
    }

    delta
}