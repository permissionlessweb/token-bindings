use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
