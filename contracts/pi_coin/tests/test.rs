// tests/test.rs
use soroban_sdk::{Env, Address, Symbol, Bytes, BytesN};
use stellar_pi_coin_sdk::PiCoinContractClient; // Assuming generated client from lib.rs

#[test]
fn test_init_and_mint() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    // Create test addresses
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    // Mock external dependencies (replace with real mocks if available)
    // In Soroban tests, simulate oracles/proofs as needed
    env.mock_all_auths(); // Allow auth for simplicity in tests
    
    // Initialize contract
    client.init(&admin);
    
    // Mint Pi Coin
    let amount = 100u64;
    let source = Symbol::new(&env, "mining");
    let coin = client.mint(&user, &amount, &source);
    
    // Assertions for upgraded struct
    assert_eq!(coin.amount, amount);
    assert_eq!(coin.owner, user);
    assert_eq!(coin.source, source);
    assert_eq!(coin.verified, true);
    assert!(!coin.proof.is_empty()); // New: Ensure mega-negation proof is generated
    
    // Additional checks for hyper-tech features
    assert_eq!(client.get_usd_value(&amount), amount * 314159); // Fixed peg
    assert!(!client.check_anomaly(&amount)); // No anomaly for small amount
    assert!(client.ai_governance_check(&Bytes::from(b"test_data"))); // Purity check (mocked)
    
    // Test transfer with proof verification
    let coin_id = BytesN::from_array(&env, &[0; 32]); // Mock coin ID (in real test, use actual hash)
    client.transfer(&user, &Address::generate(&env), &50, &coin_id);
    // Assert transfer succeeded (add more if needed)
}

#[test]
fn test_supply_cap_and_peg() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin);
    
    // Test supply cap
    let large_amount = 100_000_000_001u64; // Exceeds cap
    let result = std::panic::catch_unwind(|| {
        client.mint(&user, &large_amount, &Symbol::new(&env, "mining"));
    });
    assert!(result.is_err()); // Should panic due to cap
    
    // Test peg enforcement (mock oracle to violate peg)
    // In real test, mock oracle to return != 314159
    // For now, assume mint succeeds if peg is met
    let coin = client.mint(&user, &100, &Symbol::new(&env, "mining"));
    assert_eq!(coin.amount, 100);
}

#[test]
fn test_transfer_and_legal_tender() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    
    env.mock_all_auths();
    client.init(&admin);
    
    // Mint and transfer
    let coin = client.mint(&user1, &200, &Symbol::new(&env, "rewards"));
    let coin_id = BytesN::from_array(&env, &[1; 32]); // Mock ID
    client.transfer(&user1, &user2, &100, &coin_id);
    
    // Test global legal tender enforcement
    assert!(client.enforce_global_legal_tender(&coin_id)); // Should be true for valid coin
}
