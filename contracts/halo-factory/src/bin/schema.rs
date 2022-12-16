use cosmwasm_schema::write_api;
use haloswap::factory::{ExecuteMsg, InstantiateMsg, QueryMsg};
// use std::env::current_dir;
// use std::fs::create_dir_all;

// use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

// use haloswap::asset::PairInfo;
// use haloswap::factory::{ConfigResponse, ExecuteMsg, InstantiateMsg, PairsResponse, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
    // let mut out_dir = current_dir().unwrap();
    // out_dir.push("schema");
    // create_dir_all(&out_dir).unwrap();
    // remove_schemas(&out_dir).unwrap();

    // export_schema(&schema_for!(InstantiateMsg), &out_dir);
    // export_schema(&schema_for!(ExecuteMsg), &out_dir);
    // export_schema(&schema_for!(QueryMsg), &out_dir);
    // export_schema(&schema_for!(PairInfo), &out_dir);
    // export_schema(&schema_for!(PairsResponse), &out_dir);
    // export_schema(&schema_for!(ConfigResponse), &out_dir);
}
