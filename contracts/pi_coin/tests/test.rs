// tests/test.rs - Updated for GodHead Nexus Level with Safety and Realism
use soroban_sdk::{Env, Address, Symbol, Bytes, BytesN, events, Vec};
use pi_coin_contract::PiCoinContractClient; // Assuming generated client from updated lib.rs

#[test]
fn test_init_and_basic_setup() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let signers = Vec::from_array(&env, [admin.clone()]);
    let threshold = 1u32;
    
    env.mock_all_auths();
    let result = client.init(&admin, &signers, &threshold);
    assert!(result.is_ok());
    
    // Check events
    let events = env.events();
    assert!(events.len() > 0);
    assert_eq!(events[0].1, Symbol::new(&env, "GodHeadNexusInitialized"));
    
    // Check supply
    let supply = client.get_current_supply();
    assert_eq!(supply, Ok(0u64));
}

#[test]
fn test_mint_with_compliance_and_verifications() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let signers = Vec::from_array(&env, [admin.clone()]);
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin, &signers, &1u32).unwrap();
    
    // Register compliance for user
    client.register_compliance(&user, &true, &Symbol::new(&env, "US"), &10u32).unwrap();
    
    let amount = 100u64;
    let source = Symbol::new(&env, "mining");
    let coin_result = client.mint(&user, &amount, &source);
    assert!(coin_result.is_ok());
    let coin = coin_result.unwrap();
    
    assert_eq!(coin.amount, amount);
    assert_eq!(coin.owner, user);
    assert_eq!(coin.source, source);
    assert_eq!(coin.verified, true);
    assert!(!coin.proof.is_empty());
    
    // Check supply increase
    let supply = client.get_current_supply();
    assert_eq!(supply, Ok(amount));
    
    // Check events
    let events = env.events();
    assert!(events.iter().any(|e| e.1 == Symbol::new(&env, "GodHeadNexusMinted")));
}

#[test]
fn test_transfer_with_compliance_and_proof_check() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let signers = Vec::from_array(&env, [admin.clone()]);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin, &signers, &1u32).unwrap();
    
    // Register compliance for both users
    client.register_compliance(&user1, &true, &Symbol::new(&env, "US"), &10u32).unwrap();
    client.register_compliance(&user2, &true, &Symbol::new(&env, "ID"), &20u32).unwrap();
    
    let coin_result = client.mint(&user1, &200, &Symbol::new(&env, "rewards"));
    assert!(coin_result.is_ok());
    // Mock coin_id (in real test, use actual hash from mint)
    let coin_id = BytesN::from_array(&env, &[0; 32]);
    
    let transfer_result = client.transfer(&user1, &user2, &100, &coin_id);
    assert!(transfer_result.is_ok());
    
    // Check events
    let events = env.events();
    assert!(events.iter().any(|e| e.1 == Symbol::new(&env, "GodHeadNexusTransferred")));
    
    // Check supply unchanged
    let supply = client.get_current_supply();
    assert_eq!(supply, Ok(200));
}

#[test]
fn test_burn_and_supply_control() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let signers = Vec::from_array(&env, [admin.clone()]);
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin, &signers, &1u32).unwrap();
    
    // Register compliance
    client.register_compliance(&user, &true, &Symbol::new(&env, "US"), &10u32).unwrap();
    
    let coin_result = client.mint(&user, &500, &Symbol::new(&env, "p2p"));
    assert!(coin_result.is_ok());
    let coin_id = BytesN::from_array(&env, &[1; 32]); // Mock
    
    let burn_result = client.burn(&user, &200, &coin_id);
    assert!(burn_result.is_ok());
    
    // Check supply decrease
    let supply = client.get_current_supply();
    assert_eq!(supply, Ok(300));
    
    // Check events
    let events = env.events();
    assert!(events.iter().any(|e| e.1 == Symbol::new(&env, "GodHeadNexusBurned")));
}

#[test]
fn test_compliance_registration_and_oracle_update() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let signers = Vec::from_array(&env, [admin.clone()]);
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin, &signers, &1u32).unwrap();
    
    // Register compliance
    let compliance_result = client.register_compliance(&user, &true, &Symbol::new(&env, "US"), &10u32);
    assert!(compliance_result.is_ok());
    
    // Check events
    let events = env.events();
    assert!(events.iter().any(|e| e.1 == Symbol::new(&env, "GodHeadComplianceRegistered")));
    
    // Update oracle feed
    let oracle_result = client.update_oracle_feed(&Symbol::new(&env, "PI"), &314159);
    assert!(oracle_result.is_ok());
}

#[test]
fn test_ai_governance_vote() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let signers = Vec::from_array(&env, [admin.clone()]);
    let voter = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin, &signers, &1u32).unwrap();
    
    // AI governance vote
    let vote_result = client.ai_governance_vote(&voter, &Symbol::new(&env, "proposal1"), &true);
    assert!(vote_result.is_ok());
    
    // Check events
    let events = env.events();
    assert!(events.iter().any(|e| e.1 == Symbol::new(&env, "GodHeadAIGovernanceVoted")));
}

#[test]
fn test_interdimensional_bridge() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let signers = Vec::from_array(&env, [admin.clone()]);
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin, &signers, &1u32).unwrap();
    
    // Mock bridge registry (in real test, set via admin)
    let bridge_result = client.interdimensional_bridge(&user, &Symbol::new(&env, "mars"), &100);
    // Assume bridge not set, should err
    assert!(bridge_result.is_err());
}

#[test]
fn test_invalid_operations_with_error_handling() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let signers = Vec::from_array(&env, [admin.clone()]);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin, &signers, &1u32).unwrap();
    
    // Register compliance for user1 only
    client.register_compliance(&user1, &true, &Symbol::new(&env, "US"), &10u32).unwrap();
    
    // Mint first
    let coin_result = client.mint(&user1, &100, &Symbol::new(&env, "mining"));
    assert!(coin_result.is_ok());
    let coin_id = BytesN::from_array(&env, &[3; 32]);
    
    // Test invalid transfer (recipient not compliant)
    let transfer_result = client.transfer(&user1, &user2, &50, &coin_id);
    assert!(transfer_result.is_err());
    
    // Test invalid source
    let mint_result = client.mint(&user1, &50, &Symbol::new(&env, "invalid"));
    assert!(mint_result.is_err());
    
    // Test invalid burn (insufficient amount)
    let burn_result = client.burn(&user1, &200, &coin_id);
    assert!(burn_result.is_err());
}

#[test]
fn test_supply_cap_and_peg_enforcement() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let signers = Vec::from_array(&env, [admin.clone()]);
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin, &signers, &1u32).unwrap();
    
    // Register compliance
    client.register_compliance(&user, &true, &Symbol::new(&env, "US"), &10u32).unwrap();
    
    // Test normal mint
    let mint_result = client.mint(&user, &1000, &Symbol::new(&env, "mining"));
    assert!(mint_result.is_ok());
    let supply = client.get_current_supply();
    assert_eq!(supply, Ok(1000));
    
    // Test supply cap (mock exceed by minting large amount)
    let large_mint = client.mint(&user, &100_000_000_000u64, &Symbol::new(&env, "mining"));
    assert!(large_mint.is_err()); // Should hit cap
}
