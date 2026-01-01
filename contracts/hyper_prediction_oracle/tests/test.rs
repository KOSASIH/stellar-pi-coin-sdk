#[test]
fn test_predict_trend() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HyperPredictionOracleContract);
    let client = HyperPredictionOracleContractClient::new(&env, &contract_id);
    
    client.init(&admin);
    let input = Map::new(&env);
    input.set(Symbol::new(&env, "volatility"), 50);
    let prediction = client.predict_trend(&input);
    assert!(prediction.confidence > 0);
}
