// contracts/pi_coin/src/oracle_integration.rs
// GodHead Nexus Oracle Integration: Queries off-chain Hyper AI for predictions.
// Uses try_call to an oracle contract that fetches from the Python API.

use soroban_sdk::{contracttype, Env, Symbol, Vec, Address, Val, Error};
use crate::DataKey;

#[contracttype]
pub enum OracleRequest {
    Predict(i64),  // Input for prediction
    Evolve(Vec<(i64, i64)>),  // Feedback data for evolution
}

pub struct OracleIntegration;

impl OracleIntegration {
    // Query AI prediction via oracle
    pub fn query_ai_predict(env: &Env, input: i64) -> Result<i64, u32> {
        let oracle_addr: Address = env.storage().persistent().get(&DataKey::PegOracle)  // Reuse or add new key
            .ok_or(1002)?;  // Assume oracle contract address stored
        let request = OracleRequest::Predict(input);
        let result: Result<Val, Error> = env.try_call(
            oracle_addr,
            Symbol::new(env, "query_external_ai"),
            Vec::from_array(env, [request.into()])
        );
        match result {
            Ok(val) => {
                let prediction: i64 = val.try_into().map_err(|_| 1003)?;
                Ok(prediction.clamp(0, 100))
            }
            Err(_) => Err(1003),  // Call failed
        }
    }

    // Trigger AI evolution via oracle
    pub fn trigger_ai_evolution(env: &Env, feedback_data: Vec<(i64, i64)>) -> Result<(), u32> {
        let oracle_addr: Address = env.storage().persistent().get(&DataKey::PegOracle).ok_or(1002)?;
        let request = OracleRequest::Evolve(feedback_data);
        let result: Result<Val, Error> = env.try_call(
            oracle_addr,
            Symbol::new(env, "evolve_external_ai"),
            Vec::from_array(env, [request.into()])
        );
        result.map_err(|_| 1003)?;
        Ok(())
    }
}
