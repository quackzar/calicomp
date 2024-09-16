use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::sys::recipe::{Product, Recipe};

#[derive(Serialize, Deserialize, Default)]
pub struct Repostory {
    // TODO: Properly use IDs
    pub recipes: BTreeMap<String, Recipe>,
    pub ingredients: BTreeMap<String, Product>,
}
