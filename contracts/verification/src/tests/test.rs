#[test]
fn test_verify_origin() {
    let env = Env::default();
    let contract_id = env.register_contract(None, VerificationContract);
    let client = VerificationContractClient::new(&env, &contract_id);
    
    client.init(&admin);
    let result = client.verify_origin(&Symbol::new(&env, "mining"), &coin_id, &100, &5);
    assert!(result.is_valid);
}
