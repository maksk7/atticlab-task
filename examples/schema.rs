use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use atticlab_task::msg::{
    AllBeveragesDetails, AllBeveragesResponse, ExecuteMsg, InstantiateMsg, QueryMsg,
};
use atticlab_task::state::BeverageInfo;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(BeverageInfo), &out_dir);
    export_schema(&schema_for!(AllBeveragesDetails), &out_dir);
    export_schema(&schema_for!(AllBeveragesResponse), &out_dir);
}
