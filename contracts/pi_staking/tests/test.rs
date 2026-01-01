#[test]
fn test_stake() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PiStakingContract);
    let client = PiStakingContractClient::new(&env, &contract_id);
    
    client.init(&admin, &5);
    client.stake(&staker, &1000);
    assert_eq!(client.get_total_staked(), 1000);
}
