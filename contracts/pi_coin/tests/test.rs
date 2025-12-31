#[test]
fn test_mint() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiCoinContract);
    let client = PiCoinContractClient::new(&env, &contract_id);
    
    client.init(&admin);
    let coin = client.mint(&user, &100, &Symbol::new(&env, "mining"));
    assert_eq!(coin.amount, 100);
}
