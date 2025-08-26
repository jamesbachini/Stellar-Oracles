#![no_std]

use soroban_sdk::{ contract, contractimpl, contracterror, Env, Address, panic_with_error, Symbol, symbol_short, BytesN };
use crate::reflector::{ReflectorClient, Asset as ReflectorAsset};
use stellar_access::ownable::{self as ownable, Ownable};
use stellar_macros::{default_impl, only_owner};
use sep_41_token::TokenClient;

const TOKEN: Symbol = symbol_short!("token");

#[contract]
pub struct PriceLockContract;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    PriceTooLow = 1,
    TransferFailed = 2,
}

#[contractimpl]
impl PriceLockContract {
    pub fn __constructor(env: Env, owner: Address) {
        ownable::set_owner(&env, &owner);
        let token_address;
        if Self::is_mainnet(&env) == true {
            token_address = Address::from_str(&env, "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA"); // XLM Mainnet
        } else {
            token_address = Address::from_str(&env, "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"); // XLM Testnet
        }
        env.storage().persistent().set(&TOKEN, &token_address);
    }

    pub fn is_mainnet(env: &Env) -> bool {
        let network_id = env.ledger().network_id();
        let mainnet_id = BytesN::from_array(
            &env,
            &[0x7a, 0xc3, 0x39, 0x97, 0x54, 0x4e, 0x31, 0x75, 0xd2, 0x66, 0xbd, 0x02, 0x24, 0x39, 0xb2, 0x2c, 0xdb, 0x16, 0x50, 0x8c, 0x01, 0x16, 0x3f, 0x26,0xe5, 0xcb, 0x2a, 0x3e, 0x10, 0x45, 0xa9, 0x79]
        );
        network_id == mainnet_id
    }

    pub fn oracle_price(env: Env) -> i128 {
        let oracle_address = Address::from_str(&env, "CALI2BYU2JE6WVRUFYTS6MSBNEHGJ35P4AVCZYF3B6QOE3QKOB2PLE6M");
        let token: Address = env.storage().persistent().get(&TOKEN).unwrap();
        let reflector_client = ReflectorClient::new(&env, &oracle_address);
        let reflector_asset = ReflectorAsset::Stellar(token);
        let recent = reflector_client.lastprice(&reflector_asset);
        let price = recent.unwrap().price;
        price
    }

    pub fn testnet_price(_env: Env) -> i128 {
        999999999999999999999999999999991
    }

    #[only_owner]
    pub fn withdraw(env: Env, to: Address) {
        let token: Address = env.storage().persistent().get(&TOKEN).unwrap();
        let xlm_price;
        if Self::is_mainnet(&env) == true {
            xlm_price = Self::oracle_price(env.clone());
        } else {
            xlm_price = Self::testnet_price(env.clone());
        }
        if xlm_price < 99999999999999999999999999999999 {
            panic_with_error!(&env, Error::PriceTooLow);
        }
        let xlm_client = TokenClient::new(&env, &token);
        let balance = xlm_client.balance(&env.current_contract_address());
        xlm_client.transfer(
            &env.current_contract_address(),
            &to,
            &balance,
        );
    }
}

#[default_impl]
#[contractimpl]
impl Ownable for PriceLockContract {}

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