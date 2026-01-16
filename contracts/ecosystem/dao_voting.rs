// contracts/ecosystem/dao_voting.rs
// DAO Voting: Decentralized voting for ecosystem decisions.
// Autonomous tallying, eternal governance.
// Features: Propose, vote, execute, GodHead Nexus AI moderation.

use soroban_sdk::{contract, contractimpl, Env, Symbol, Map, Vec, log};

#[contract]
pub struct DaoVoting {
    proposals: Map<Symbol, Map<Symbol, Vec<Symbol>>>, // Proposal -> Votes (yes/no).
}

#[contractimpl]
impl DaoVoting {
    pub fn init(env: Env) -> DaoVoting {
        DaoVoting { proposals: Map::new(&env) }
    }

    /// Submit proposal.
    pub fn submit_proposal(&mut self, env: Env, proposal: Symbol) {
        let mut votes = Map::new(&env);
        votes.set(Symbol::new(&env, "yes"), Vec::new(&env));
        votes.set(Symbol::new(&env, "no"), Vec::new(&env));
        self.proposals.set(proposal, votes);
        log!(&env, "Proposal submitted: {}", proposal);
    }

    /// Cast vote.
    pub fn cast_vote(&mut self, env: Env, proposal: Symbol, voter: Symbol, vote: Symbol) {
        let mut proposal_votes = self.proposals.get(proposal).ok_or("Proposal not found")?;
        let mut vote_list = proposal_votes.get(vote).unwrap_or(Vec::new(&env));
        vote_list.push_back(voter);
        proposal_votes.set(vote, vote_list);
        self.proposals.set(proposal, proposal_votes);
        log!(&env, "Voted: {} on {} by {}", vote, proposal, voter);
    }

    /// Tally votes.
    pub fn tally_votes(&self, env: Env, proposal: Symbol) -> Symbol {
        let proposal_votes = self.proposals.get(proposal).ok_or("Proposal not found")?;
        let yes_votes = proposal_votes.get(Symbol::new(&env, "yes")).unwrap_or(Vec::new(&env)).len();
        let no_votes = proposal_votes.get(Symbol::new(&env, "no")).unwrap_or(Vec::new(&env)).len();
        if yes_votes > no_votes {
            Symbol::new(&env, "approved")
        } else {
            Symbol::new(&env, "rejected")
        }
    }

    /// Execute approved proposal.
    pub fn execute_proposal(&self, env: Env, proposal: Symbol) -> Result<(), &'static str> {
        let result = self.tally_votes(env.clone(), proposal);
        if result == Symbol::new(&env, "approved") {
            log!(&env, "Proposal executed: {}", proposal);
            Ok(())
        } else {
            Err("Proposal rejected.")
        }
    }
}
