use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CanonicalAddr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub terraswap_factory: CanonicalAddr,
    pub loop_factory: CanonicalAddr,
    pub astroport_factory: CanonicalAddr,
}

pub const CONFIG: Item<Config> = Item::new("config");
