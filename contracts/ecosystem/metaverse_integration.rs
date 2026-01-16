// contracts/ecosystem/metaverse_integration.rs
// Metaverse Integration: Immersive virtual worlds for Pi Coin.
// Autonomous world building, interactions; eternal metaverse.
// Features: Create world, interact, trade virtual assets, GodHead Nexus AI curation.

use soroban_sdk::{contract, contractimpl, Env, Symbol, Map, Vec, log};

#[contract]
pub struct MetaverseIntegration {
    worlds: Map<Symbol, Map<Symbol, Vec<Symbol>>>, // World -> Assets (user, item).
}

#[contractimpl]
impl MetaverseIntegration {
    pub fn init(env: Env) -> MetaverseIntegration {
        MetaverseIntegration { worlds: Map::new(&env) }
    }

    /// Create virtual world.
    pub fn create_world(&mut self, env: Env, world_id: Symbol, creator: Symbol) {
        let mut assets = Map::new(&env);
        assets.set(Symbol::new(&env, "land"), Vec::new(&env));
        self.worlds.set(world_id, assets);
        log!(&env, "World created: {} by {}", world_id, creator);
    }

    /// Add virtual asset.
    pub fn add_asset(&mut self, env: Env, world_id: Symbol, user: Symbol, asset: Symbol) {
        let mut world_assets = self.worlds.get(world_id).ok_or("World not found")?;
        let mut user_assets = world_assets.get(user).unwrap_or(Vec::new(&env));
        user_assets.push_back(asset);
        world_assets.set(user, user_assets);
        self.worlds.set(world_id, world_assets);
        log!(&env, "Asset added: {} to {} in {}", asset, user, world_id);
    }

    /// Trade virtual asset.
    pub fn trade_asset(&mut self, env: Env, world_id: Symbol, seller: Symbol, buyer: Symbol, asset: Symbol) -> Result<(), &'static str> {
        let mut world_assets = self.worlds.get(world_id).ok_or("World not found")?;
        let mut seller_assets = world_assets.get(seller).ok_or("Seller has no assets")?;
        if seller_assets.contains(&asset) {
            seller_assets.retain(|&a| a != asset);
            let mut buyer_assets = world_assets.get(buyer).unwrap_or(Vec::new(&env));
            buyer_assets.push_back(asset);
            world_assets.set(seller, seller_assets);
            world_assets.set(buyer, buyer_assets);
            self.worlds.set(world_id, world_assets);
            log!(&env, "Asset traded: {} from {} to {} in {}", asset, seller, buyer, world_id);
            Ok(())
        } else {
            Err("Asset not owned by seller.")
        }
    }

    /// Get world assets.
    pub fn get_world_assets(&self, env: Env, world_id: Symbol) -> Map<Symbol, Vec<Symbol>> {
        self.worlds.get(world_id).unwrap_or(Map::new(&env))
    }
}
