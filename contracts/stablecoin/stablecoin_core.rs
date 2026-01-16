// contracts/stablecoin/stablecoin_core.rs
// Stablecoin Core: Central mechanics for Pi Coin stability.
// Supply management, eternal balance.
// Features: Mint core, burn core, transfer core, GodHead Nexus AI oversight.

use soroban_sdk::{contract, contractimpl, Env, Symbol, Map, log};

#[contract]
pub struct StablecoinCore {
    total_supply: i128,
    balances: Map<Symbol, i128>, // User -> Balance.
}

#[contractimpl]
impl StablecoinCore {
    pub fn init(env: Env) -> StablecoinCore {
        StablecoinCore { total_supply: 0, balances: Map::new(&env) }
    }

    /// Mint core PI.
    pub fn mint_core(&mut self, env: Env, to: Symbol, amount: i128) {
        let current = self.balances.get(to).unwrap_or(0);
        self.balances.set(to, current + amount);
        self.total_supply += amount;
        log!(&env, "Core minted: {} PI to {}", amount, to);
    }

    /// Burn core PI.
    pub fn burn_core(&mut self, env: Env, from: Symbol, amount: i128) -> Result<(), &'static str> {
        let current = self.balances.get(from).unwrap_or(0);
        if current >= amount {
            self.balances.set(from, current - amount);
            self.total_supply -= amount;
            log!(&env, "Core burned: {} PI from {}", amount, from);
            Ok(())
        } else {
            Err("Insufficient balance.")
        }
    }

    /// Transfer core PI.
    pub fn transfer_core(&mut self, env: Env, from: Symbol, to: Symbol, amount: i128) -> Result<(), &'static str> {
        self.burn_core(env.clone(), from, amount)?;
        self.mint_core(env, to, amount);
        Ok(())
    }

    /// Get balance.
    pub fn get_balance(&self, env: Env, user: Symbol) -> i128 {
        self.balances.get(user).unwrap_or(0)
    }

    /// Get total supply.
    pub fn get_total_supply(&self, env: Env) -> i128 {
        self.total_supply
    }
}
