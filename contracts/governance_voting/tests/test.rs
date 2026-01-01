#[test]
fn test_create_proposal() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GovernanceVotingContract);
    let client = GovernanceVotingContractClient::new(&env, &contract_id);
    
    client.init(&admin);
    let proposal_id = client.create_proposal(&creator, &Symbol::new(&env, "increase_rewards"), &80, &70, &90);
    let proposal = client.get_proposal(&proposal_id);
    assert_eq!(proposal.description, Symbol::new(&env, "increase_rewards"));
}
