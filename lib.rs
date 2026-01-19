#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, Symbol, Map,
};

/// -------------------------------
/// DATA TYPES
/// -------------------------------

#[contracttype]
#[derive(Clone)]
pub struct Merchant {
    pub name: Symbol,
    pub products: Map<Symbol, u64>, // product -> adjusted price
}

#[contracttype]
#[derive(Clone)]
pub struct ServiceProvider {
    pub name: Symbol,
    pub services: Map<Symbol, u64>, // service -> price
}

#[contracttype]
#[derive(Clone)]
pub struct EcosystemAnalytics {
    pub total_transactions: u64,
    pub average_amount: u64,
    pub anomalies_detected: u32,
    pub utilization_percent: u32,
}

/// -------------------------------
/// STORAGE KEYS
/// -------------------------------

#[contracttype]
pub enum DataKey {
    Admin,
    Merchants,
    ServiceProviders,
    OracleData,
    Analytics,
}

/// -------------------------------
/// CONTRACT
/// -------------------------------

#[contract]
pub struct EcosystemContract;

/// -------------------------------
/// IMPLEMENTATION
/// -------------------------------

#[contractimpl]
impl EcosystemContract {

    /// Initialize ecosystem
    pub fn init(env: Env, admin: Address) {
        admin.require_auth();

        let merchants: Map<Symbol, Merchant> = Map::new(&env);
        let providers: Map<Symbol, ServiceProvider> = Map::new(&env);
        let oracle_data: Map<Symbol, i128> = Map::new(&env);

        oracle_data.set(Symbol::short("market_trend"), 1_000_000_043);

        let analytics = EcosystemAnalytics {
            total_transactions: 0,
            average_amount: 0,
            anomalies_detected: 0,
            utilization_percent: 0,
        };

        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::Merchants, &merchants);
        env.storage().persistent().set(&DataKey::ServiceProviders, &providers);
        env.storage().persistent().set(&DataKey::OracleData, &oracle_data);
        env.storage().persistent().set(&DataKey::Analytics, &analytics);
    }

    /// -------------------------------
    /// REGISTER MERCHANT
    /// -------------------------------
    pub fn register_merchant(
        env: Env,
        name: Symbol,
        products: Map<Symbol, u64>,
    ) {
        let mut merchants: Map<Symbol, Merchant> =
            env.storage().persistent().get(&DataKey::Merchants).unwrap();

        let mut adjusted_products = Map::new(&env);

        for (product, price) in products.iter() {
            let adjusted_price = price / 30; // price normalization
            adjusted_products.set(product, adjusted_price);
        }

        let merchant = Merchant {
            name: name.clone(),
            products: adjusted_products,
        };

        merchants.set(name, merchant);
        env.storage().persistent().set(&DataKey::Merchants, &merchants);
    }

    /// -------------------------------
    /// REGISTER SERVICE PROVIDER
    /// -------------------------------
    pub fn register_service_provider(
        env: Env,
        name: Symbol,
        services: Map<Symbol, u64>,
    ) {
        let mut providers: Map<Symbol, ServiceProvider> =
            env.storage().persistent().get(&DataKey::ServiceProviders).unwrap();

        let provider = ServiceProvider {
            name: name.clone(),
            services,
        };

        providers.set(name, provider);
        env.storage().persistent().set(&DataKey::ServiceProviders, &providers);
    }

    /// -------------------------------
    /// UPDATE ANALYTICS
    /// -------------------------------
    pub fn update_analytics(
        env: Env,
        tx_count: u64,
        avg_amount: u64,
        anomalies: u32,
        utilization: u32,
    ) {
        let mut analytics: EcosystemAnalytics =
            env.storage().persistent().get(&DataKey::Analytics).unwrap();

        analytics.total_transactions += tx_count;
        analytics.average_amount = avg_amount;
        analytics.anomalies_detected += anomalies;
        analytics.utilization_percent = utilization;

        env.storage().persistent().set(&DataKey::Analytics, &analytics);
    }

    /// -------------------------------
    /// READ PI VALUE (STATIC ORACLE)
    /// -------------------------------
    pub fn pi_value() -> i128 {
        314_159
    }
}
