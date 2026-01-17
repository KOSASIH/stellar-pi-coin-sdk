// contracts/interplanetary_economy/interplanetary_core.rs
// Interplanetary Core: Core mechanics for interplanetary Pi Coin economy.
// Planetary supply management, eternal cosmic balance.
// Features: Planetary mint, burn, transfer, GodHead Nexus AI oversight.

use soroban_sdk::{contract, contractimpl, Env, Symbol, Map, log};

#[contract]
pub struct InterplanetaryCore {
    planetary_supply: Map<Symbol, i128>, // Planet -> Supply.
    total_supply: i128, // 100,000,000,000.
}

#[contractimpl]
impl InterplanetaryCore {
    pub fn init(env: Env) -> InterplanetaryCore {
        InterplanetaryCore { planetary_supply: Map::new(&env), total_supply: 100000000000 }
    }

    /// Mint planetary PI.
    pub fn mint_planetary(&mut self, env: Env, planet: Symbol, amount: i128) {
        let current = self.planetary_supply.get(planet).unwrap_or(0);
        self.planetary_supply.set(planet, current + amount);
        log!(&env, "Planetary minted: {} PI on {}", amount, planet);
    }

    /// Burn planetary PI.
    pub fn burn_planetary(&mut self, env: Env, planet: Symbol, amount: i128) -> Result<(), &'static str> {
        let current = self.planetary_supply.get(planet).unwrap_or(0);
        if current >= amount {
            self.planetary_supply.set(planet, current - amount);
            log!(&env, "Planetary burned: {} PI on {}", amount, planet);
            Ok(())
        } else {
            Err("Insufficient planetary supply.")
        }
    }

    /// Transfer interplanetary PI.
    pub fn transfer_interplanetary(&mut self, env: Env, from_planet: Symbol, to_planet: Symbol, amount: i128) -> Result<(), &'static str> {
        self.burn_planetary(env.clone(), from_planet, amount)?;
        self.mint_planetary(env, to_planet, amount);
        Ok(())
    }

    /// Get planetary supply.
    pub fn get_planetary_supply(&self, env: Env, planet: Symbol) -> i128 {
        self.planetary_supply.get(planet).unwrap_or(0)
    }
}
