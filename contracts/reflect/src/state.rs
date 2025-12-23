use cw_storage_plus::{Item, Map};

use cosmwasm_std::{Addr, Reply};

const CONFIG_KEY: &[u8] = b"config";
const RESULT_PREFIX: &[u8] = b"result";

#[cosmwasm_schema::cw_serde]
pub struct State {
    pub owner: Addr,
}

pub const CONFIG: Item<State> = Item::new("config");
pub const RESULT: Map<&[u8; 8], Reply> = Map::new("result");
