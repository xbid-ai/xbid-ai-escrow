/*
    Copyright (c) 2026 XBID LABS LLC

    This file is part of XBID-AI project.
    Licensed under the MIT License.
    Author: Fred Kyung-jin Rezeau (오경진 吳景振) <hello@kyungj.in>
*/

use crate::*;
use crate::emission;
use crate::types::EmissionParams;
use soroban_sdk::{testutils::{Address as _, Ledger}, token, Address, Bytes, Env};

#[test]
#[should_panic]
fn test_escrow_provenance() {
    let params = EmissionParams::default();
    let env = Env::default();
    let contract_id = env.register(Escrow, ());
    let client = EscrowClient::new(&env, &contract_id);
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone()).address();
    let token_admin_client = token::StellarAssetClient::new(&env, &token_id);
    let admin = Address::generate(&env);
    let fund = Address::generate(&env);
    let rewards = Address::generate(&env);
    let ledger: u32 = 1_000;

    env.mock_all_auths();
    env.ledger().set_sequence_number(ledger);
    token_admin_client.mint(&admin, &(params.total_supply - 1));
    client.initialize(&admin, &token_id, &fund, &rewards, &ledger);
}


#[test]
fn test_escrow_release() {
    let params = EmissionParams::default();
    let env = Env::default();
    let allocation = params.allocation;
    let contract_id = env.register(Escrow, ());
    let client = EscrowClient::new(&env, &contract_id);
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone()).address();
    let token = token::Client::new(&env, &token_id);
    let token_admin_client = token::StellarAssetClient::new(&env, &token_id);
    let admin = Address::generate(&env);
    let fund = Address::generate(&env);
    let rewards = Address::generate(&env);
    let mut current_ledger = 1000u32;
    let mut cumulative_fund = allocation;
    let mut cumulative_rewards = 0i128;

    const REF: [(u32, i128, i128, i128); 20] = [
        ( 7,          1553779572,           15537795,         1538241777),
        (15,          3329525995,           33295259,         3296230736),
        (21,          4661332591,           46613325,         4614719266),
        (37,          8212813301,           82128133,         8130685168),
        (42,          9322636255,           93226362,         9229409893),
        (13,          2885574294,           28855742,         2856718552),
        (28,          6215077323,           62150773,         6152926550),
        ( 9,          1997701751,           19977017,         1977724734),
        (51,         11320294538,          113202945,        11207091593),
        (19,          4217357945,           42173579,         4175184366),
        (33,          7324876225,           73248762,         7251627463),
        ( 8,          1775725921,           17757259,         1757968662),
        (44,          9766481061,           97664810,         9668816251),
        (11,          2441617224,           24416172,         2417201052),
        (29,          6436985030,           64369850,         6372615180),
        (17,          3773401087,           37734010,         3735667077),
        (38,          8434650745,           84346507,         8350304238),
        (12,          2663570903,           26635709,         2636935194),
        (25,          5549101397,           55491013,         5493610384),
        (16,          3551421597,           35514215,         3515907382)
    ];

    env.mock_all_auths();

    token_admin_client.mint(&admin, &params.total_supply);
    env.ledger().set_sequence_number(current_ledger);

    client.initialize(&admin, &token_id, &fund, &rewards, &current_ledger);

    for (increment, expected_emissions, expected_fund, expected_rewards) in REF.iter() {
        current_ledger += increment;
        env.ledger().set_sequence_number(current_ledger);

        let released = client.release();
        cumulative_fund += expected_fund;
        cumulative_rewards += expected_rewards;

        assert_eq!(released, *expected_emissions, "emissions mismatch at ledger {}", current_ledger);
        assert_eq!(token.balance(&fund), cumulative_fund, "fund mismatch at ledger {}", current_ledger);
        assert_eq!(token.balance(&rewards), cumulative_rewards, "rewards mismatch at ledger {}", current_ledger);
    }
}

#[test]
fn test_emission_delta_clamp() {
    assert_eq!(emission::delta(1000, 1000, 0), 0);
    assert_eq!(emission::delta(1000,  999, 0), 0);
}

#[test]
fn test_emission_curve_50y() {
    // (origin, target, expected)
    const REF: &[(u32, u32, i128)] = &[
        (1000001, 7307201, 1225000178519665),
        (7307201, 13614401, 952777748779824),
        (13614401, 19921601, 762222202045097),
        (19921601, 26228801, 623636349125320),
        (26228801, 32536001, 519696958977724),
        (32536001, 38843201, 439743581648575),
        (38843201, 45150401, 376923070695931),
        (45150401, 51457601, 326666661801014),
        (51457601, 57764801, 285833329480517),
        (57764801, 64072001, 252205879267021),
        (64072001, 70379201, 224183004039740),
        (70379201, 76686401, 200584793285109),
        (76686401, 82993601, 180526314115610),
        (82993601, 89300801, 163333331948692),
        (89300801, 95608001, 148484847333093),
        (95608001, 101915201, 135573121567047),
        (101915201, 108222401, 124275361511096),
        (108222401, 114529601, 114333332653259),
        (114529601, 120836801, 105538460964333),
        (120836801, 127144001, 97720797235104),
        (127144001, 133451201, 90740740329264),
        (133451201, 139758401, 84482758271803),
        (139758401, 146065601, 78850574416783),
        (146065601, 152372801, 73763440609460),
        (152372801, 158680001, 69153225594211),
        (158680001, 164987201, 64962121032880),
        (164987201, 171294401, 61140819813471),
        (171294401, 177601601, 57647058697099),
        (177601601, 183908801, 54444444339143),
        (183908801, 190216001, 51501501414503),
        (190216001, 196523201, 48790896088207),
        (196523201, 202830401, 46288798863086),
        (202830401, 209137601, 43974358929106),
        (209137601, 215444801, 41829268257933),
        (215444801, 221752001, 39837398348407),
        (221752001, 228059201, 37984496106476),
        (228059201, 234366401, 36257928107858),
        (234366401, 240673601, 34646464642075),
        (240673601, 246980801, 33140096619350),
        (246980801, 253288001, 31729879746689),
        (253288001, 259595201, 30407801428277),
        (259595201, 265902401, 29166666680119),
        (265902401, 272209601, 28000000016613),
        (272209601, 278516801, 26901960803689),
        (278516801, 284824001, 25867270006702),
        (284824001, 291131201, 24891146613143),
        (291131201, 297438401, 23969252296844),
        (297438401, 303745601, 23097643124926),
        (303745601, 310052801, 22272727301371),
        (310052801, 316360001, 21491228099988),
    ];

    let mut realized: i128 = 0;
    let mut prev: u32 = 1000001;
    for (origin, target, expected) in REF {
        assert_eq!(*origin, prev, "not contiguous {} -> {}", origin, target);
        let delta = emission::delta(1000000u32, *target, realized);
        assert_eq!(delta, *expected, "mismatch for {} -> {}", origin, target);
        realized += delta;
        prev = *target;
    }
}

#[test]
fn test_emission_curve_random() {
    // (origin, target, expected)
    const REF: &[(u32, u32, i128)] = &[
        (1000001, 7245742, 1214542812194988),
        (7245742, 11360155, 648023432282007),
        (11360155, 13953996, 360540391457726),
        (13953996, 19284013, 646838319810812),
        (19284013, 24485772, 532976297857110),
        (24485772, 28748700, 379285504250527),
        (28748700, 33494324, 372604699607498),
        (33494324, 39530970, 412160626374234),
        (39530970, 44499666, 296649237158480),
        (44499666, 46494862, 109866856451485),
        (46494862, 51236308, 242456231234062),
        (51236308, 53605115, 112228673844697),
        (53605115, 59426455, 253780692604836),
        (59426455, 64655398, 204589401025759),
        (64655398, 66664938, 73443312296188),
        (66664938, 71180429, 155610660892202),
        (71180429, 74085841, 93787122144640),
        (74085841, 77963138, 118150404939024),
        (77963138, 83495154, 156049186763803),
        (83495154, 85250662, 46709473321387),
        (85250662, 87068219, 47033474213504),
        (87068219, 91863944, 118021735598869),
        (91863944, 94817091, 68594470773230),
        (94817091, 96403020, 35639139647261),
        (96403020, 101906529, 117630905323102),
        (101906529, 105419755, 70533714283413),
        (105419755, 109457372, 77050881793171),
        (109457372, 114176008, 85035235542048),
        (114176008, 119268324, 86226021006600),
        (119268324, 124254211, 79341360364509),
        (124254211, 129875579, 83969971770704),
        (129875579, 133127561, 46131706555569),
        (133127561, 137979765, 65752303447889),
        (137979765, 144251947, 79949097345888),
        (144251947, 147585233, 40348411274738),
        (147585233, 150053309, 28978236718221),
        (150053309, 154583177, 51303900656581),
        (154583177, 158096992, 38205767634427),
        (158096992, 161090810, 31514649463912),
        (161090810, 162997860, 19597458474839),
        (162997860, 166148952, 31599066090100),
        (166148952, 168936864, 27174233898773),
        (168936864, 172627697, 34894997226462),
        (172627697, 177859732, 47469189804365),
        (177859732, 183295188, 46991646096508),
        (183295188, 188647993, 44123287160296),
        (188647993, 193420318, 37654711497020),
        (193420318, 196378669, 22588698430470),
        (196378669, 202043223, 41731024353275),
        (202043223, 205516115, 24644812619312),
    ];

    let mut realized: i128 = 0;
    let mut prev: u32 = 1000001;
    for (origin, target, expected) in REF {
        assert_eq!(*origin, prev, "not contiguous {} -> {}", origin, target);
        let delta = emission::delta(1000000u32, *target, realized);
        assert_eq!(delta, *expected, "mismatch for {} -> {}", origin, target);
        realized += delta;
        prev = *target;
    }
}

#[test]
fn test_invariants() {
    let params = EmissionParams::default();
    let env = Env::default();
    let contract_id = env.register(Escrow, ());
    let client = EscrowClient::new(&env, &contract_id);
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone()).address();
    let token = token::Client::new(&env, &token_id);
    let token_admin_client = token::StellarAssetClient::new(&env, &token_id);
    let admin = Address::generate(&env);
    let fund = Address::generate(&env);
    let rewards = Address::generate(&env);
    let ledger: u32 = 1_000;

    env.mock_all_auths();
    env.ledger().set_sequence_number(ledger);
    token_admin_client.mint(&admin, &params.total_supply);

    client.initialize(&admin, &token_id, &fund, &rewards, &ledger);

    // Check allocation payment.
    let allocation = params.allocation;
    assert_eq!(token.balance(&fund), allocation);
    assert_eq!(token.balance(&contract_id), params.total_supply - allocation);

    // Release.
    let r0 = client.release();
    assert_eq!(r0, 0);

    // Release at ledger + 1000.
    env.ledger().set_sequence_number(ledger + 1_000);

    let r1 = client.release();
    assert!(r1 > 0);

    // Balances.
    let fund_balance = token.balance(&fund);
    let rewards_balance = token.balance(&rewards);
    let emitted = fund_balance + rewards_balance - allocation;
    let expected_fund = (emitted * params.fund_bps) / 10_000; // 1%
    let expected_rewards = emitted - expected_fund;       // 99%

    // soroban_sdk::log!(&env, "fund balance = {}", fund_balance - allocation);
    // soroban_sdk::log!(&env, "rewards balance = {}", rewards_balance);
    assert_eq!(fund_balance - allocation, expected_fund, "fund should receive 1%");
    assert_eq!(rewards_balance, expected_rewards, "rewards should receive 99%");

    // Idempotence.
    env.ledger().set_sequence_number(ledger + 2_001);

    let r3 = client.release();
    let r4 = client.release();

    assert!(r3 > 0);
    assert_eq!(r4, 0);
}

#[test]
#[should_panic]
fn test_double_init() {
    let env = Env::default();
    let contract_id = env.register(Escrow, ());
    let client = EscrowClient::new(&env, &contract_id);
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone()).address();
    let admin = Address::generate(&env);
    let fund = Address::generate(&env);
    let rewards = Address::generate(&env);
    let ledger: u32 = 1_000;

    env.mock_all_auths();
    env.ledger().set_sequence_number(ledger);
    client.initialize(&admin, &token_id, &fund, &rewards, &ledger);
    client.initialize(&admin, &token_id, &fund, &rewards, &ledger);
}

#[test]
fn test_upgrade() {
    let params = EmissionParams::default();
    let env = Env::default();
    let contract_id = env.register(Escrow, ());
    let client = EscrowClient::new(&env, &contract_id);
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone()).address();
    let token_admin_client = token::StellarAssetClient::new(&env, &token_id);
    let admin = Address::generate(&env);
    let fund = Address::generate(&env);
    let rewards = Address::generate(&env);
    let start_ledger = 1_000u32;
    let wasm = include_bytes!("../../../target/wasm32-unknown-unknown/release/escrow.wasm");
    let wasm_hash = env.deployer().upload_contract_wasm(Bytes::from_slice(&env, wasm));

    env.mock_all_auths();
    env.ledger().set_sequence_number(start_ledger);
    token_admin_client.mint(&admin, &params.total_supply);
    client.initialize(&admin, &token_id, &fund, &rewards, &start_ledger);
    client.upgrade(&wasm_hash);
    env.ledger().set_sequence_number(start_ledger + 1_000);
    assert!(client.release() >= 0);
}
