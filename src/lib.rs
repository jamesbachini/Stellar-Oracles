#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, vec, Env, String, Vec, Symbol, Address, contracterror, token};
use crate::reflector::{ReflectorClient, Asset as ReflectorAsset};

#[contract]
pub struct OracleExampleContract;

#[contractimpl]
impl OracleExampleContract {
    pub fn get_price(env: Env) -> i128 {
        let oracle_address = Address::from_str(&env, "CALI2BYU2JE6WVRUFYTS6MSBNEHGJ35P4AVCZYF3B6QOE3QKOB2PLE6M");
        let token = Address::from_str(&env, "CB23WRDQWGSP6YPMY4UV5C4OW5CBTXKYN3XEATG7KJEZCXMJBYEHOUOV"); // KALE
        let reflector_client = ReflectorClient::new(&env, &oracle_address);
        let reflector_asset = ReflectorAsset::Stellar(token);
        let recent = reflector_client.lastprice(&reflector_asset);
        let price = recent.unwrap().price;
        price
    }
}

mod reflector {
    use soroban_sdk::{contracttype, Address, Env, Symbol, Vec};

    #[soroban_sdk::contractclient(name = "ReflectorClient")]
    pub trait Contract {
        fn base(e: Env) -> Asset;
        fn assets(e: Env) -> Vec<Asset>;
        fn decimals(e: Env) -> u32;
        fn price(e: Env, asset: Asset, timestamp: u64) -> Option<PriceData>;
        fn lastprice(e: Env, asset: Asset) -> Option<PriceData>;
        fn prices(e: Env, asset: Asset, records: u32) -> Option<Vec<PriceData>>;
        fn x_last_price(e: Env, base_asset: Asset, quote_asset: Asset) -> Option<PriceData>;
        fn x_price(e: Env, base_asset: Asset, quote_asset: Asset, timestamp: u64) -> Option<PriceData>;
        fn x_prices(e: Env, base_asset: Asset, quote_asset: Asset, records: u32) -> Option<Vec<PriceData>>;
        fn twap(e: Env, asset: Asset, records: u32) -> Option<i128>;
        fn x_twap(e: Env, base_asset: Asset, quote_asset: Asset, records: u32) -> Option<i128>;
        fn resolution(e: Env) -> u32;
        fn period(e: Env) -> Option<u64>;
        fn last_timestamp(e: Env) -> u64;
        fn version(e: Env) -> u32;
        fn admin(e: Env) -> Option<Address>;
    }

    #[contracttype(export = false)]
    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Asset {
        Stellar(Address),
        Other(Symbol),
    }

    #[contracttype(export = false)]
    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct PriceData {
        pub price: i128,
        pub timestamp: u64,
    }

    #[soroban_sdk::contracterror(export = false)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Error {
        AlreadyInitialized = 0,
        Unauthorized = 1,
        AssetMissing = 2,
        AssetAlreadyExists = 3,
        InvalidConfigVersion = 4,
        InvalidTimestamp = 5,
        InvalidUpdateLength = 6,
        AssetLimitExceeded = 7,
    }
}